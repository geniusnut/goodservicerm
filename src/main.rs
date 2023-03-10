mod spmc;

use std::collections::HashMap;
use std::error::Error;
use std::io;
use crossterm::event::{EnableMouseCapture, Event, KeyCode, DisableMouseCapture};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::{backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, Paragraph}, Terminal, Frame};
use tui::text::Text;
use tui_tree_widget::Tree;
use crate::spmc::{build_tree, get_item, Goods, read_spm, StatefulTree, vec_2_code};


struct App<'a> {
    goods_map: HashMap<String, Goods>,
    tree: StatefulTree<'a>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let goods_list = read_spm("spmc.csv");
        let tree = build_tree(&goods_list);
        Self {
            goods_map: goods_list.into_iter().map(|g|(g.code.clone(), g)).collect(),
            tree
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    // Terminal Initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f|ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('\n' | ' ') => app.tree.toggle(),
                KeyCode::Left => app.tree.left(),
                KeyCode::Right => app.tree.right(),
                KeyCode::Down => app.tree.down(),
                KeyCode::Up => app.tree.up(),
                KeyCode::Home => app.tree.first(),
                KeyCode::End => app.tree.last(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
        ].as_ref())
        .split(f.size());

    let p = Paragraph::new(format!("Tree Widget", ));
    // let p = Paragraph::new([Text::raw(format!("Tree Widget {:?}", app.tree.state))].iter());
    f.render_widget(p, chunks[0]);
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[1]);

        let tree = Tree::new(app.tree.items.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL).title("税收分类编码")
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(tree, chunks[0], &mut app.tree.state);

        let paragraph = Paragraph::new(Text::raw(
            format!("opened: {:?}, selected: {:?}, {:?}",
                    &app.tree.state.get_all_opened(),
                    &app.tree.state.selected(),
                    &app.goods_map.get( vec_2_code(&app.tree.state.selected()).as_str())
            )))
            .block(Block::default().borders(Borders::ALL).title("详情"));
        f.render_widget(paragraph, chunks[1]);
    }
}