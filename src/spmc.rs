use std::thread::current;
use csv::Reader;
use tui_tree_widget::{TreeIdentifier, TreeItem, TreeState};


const CODE_LEN: usize = 19;

struct GoodsTreeItem<'a> {
    item: TreeItem<'a>,
    goods: Goods,
}

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
pub struct Goods {
    pub code: String,
    pub hwlwmc: String,
    pub spfwfljc: String,
    pub desc: String,
}

pub fn build_tree<'a>(goods_list: &Vec<Goods>) -> StatefulTree<'a> {
    let mut first_items: Vec<TreeItem> = vec![];
    for goods in goods_list {
        let code = String::from("0") + goods.code.as_str();
        let mut p_node:  Option<&mut TreeItem> = None;
        for i in 0..10 {
            let idx = &code[2*i..2*i+2].parse::<usize>().unwrap();
            if *idx == 0 {
                break
            }
            if p_node.is_none() {
                if first_items.get(*idx - 1).is_some() {
                    p_node = first_items.get_mut(*idx -1);
                } else {
                    first_items.push(TreeItem::new(goods.hwlwmc.clone(), vec![]));
                    break
                }
            } else {
                let c_node = p_node.unwrap();
                if c_node.child(*idx - 1).is_some() {
                    p_node = c_node.child_mut(*idx - 1);
                } else {
                    c_node.add_child(TreeItem::new(goods.hwlwmc.clone(), vec![]));
                    break
                }
            }
        }
    }
    StatefulTree::with_items(first_items)
}

pub fn code_2_vec(code: &String) -> Vec<u8> {
    let v:Vec<u8> = vec![0;10];
    assert_eq!(code.len(), CODE_LEN);
    for i in 0..10 {
        let idx = &code[2*i .. 2*i+2].parse::<usize>().unwrap();
    }
    v
}

pub fn vec_2_code(v: &Vec<usize>) -> String {
    if v.is_empty() {
        return String::new();
    }
    let vec = v.to_vec();
    let mut c: String = vec.iter().map(|d| format!("{:02}", d+1)).collect();
    c.remove(0);
    format!("{:0<19}", c)
}

pub fn get_item<'a>(items: &'a [TreeItem<'a>], current: TreeIdentifier) -> Option<&'a TreeItem<'a>>{
    let idx = current.to_vec();

    let mut tree_item: Option<&TreeItem> = None;
    let mut children = items;

    for (_, i) in idx.iter().enumerate() {
        tree_item = children.get(*i);
        children = &*tree_item.unwrap().children();
    }
    return tree_item
}

pub fn read_spm(f_path: &str) -> Vec<Goods> {
    let mut rdr = Reader::from_path(f_path).unwrap();
    let iter = rdr.deserialize();
    return iter.into_iter().map(|item| item.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use tui_tree_widget::TreeIdentifier;
    use crate::spmc::{build_tree, vec_2_code};

    #[derive(Debug)]
    struct Tree {
        children: Vec<Tree>,
    }

    impl Tree {
        fn new<Children> (children: Children) -> Self
        where Children: Into<Vec<Tree>>
        {
            Self {
                children: children.into()
            }
        }

        pub fn children(&self) -> &[Tree] {
            &self.children
        }

        pub fn add_child(&mut self, child: Tree) {
            self.children.push(child);
        }
    }

    fn build_tree_v() -> Vec<Tree> {
        let mut dummy_root = Tree::new(vec![]);
        for i in 0..3 {
            dummy_root.add_child(Tree::new(vec![]))
        }
        dummy_root.children
    }

    #[test]
    fn test_build_tree() {
        let t = build_tree_v();
        println!("t: {:?}", t);
    }

    #[test]
    fn test_vec_2_code() {
        let c = vec_2_code(&vec![]);
        println!("c: {}", c);
    }
}
