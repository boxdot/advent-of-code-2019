use itertools::Itertools;
use itertools::EitherOrBoth;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct Node {
    pub label: String,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(l: &str) -> Self {
        Self {
            label: l.into(),
            parent: None,
            children: vec![],
        }
    }
}

pub struct Tree {
    pub root: Rc<RefCell<Node>>,
    pub node_by_label: HashMap<String, Rc<RefCell<Node>>>,
}

impl Tree {
    pub fn apply(&self, on_node: &mut impl FnMut(&Node, u32)) {
        Self::apply_impl(&self.root.as_ref().borrow(), 0, on_node);
    }

    fn apply_impl(node: &Node, depth: u32, on_node: &mut impl FnMut(&Node, u32)) {
        on_node(node, depth);
        for child in node.children.iter() {
            Self::apply_impl(&child.as_ref().borrow(), depth + 1, on_node);
        }
    }
}

pub fn calculate_sum_on_depths(tree: &Tree) -> u32 {
    let mut result = 0;
    tree.apply(&mut |_, depth| {
        result += depth;
    });
    result
}

fn get_path_to_root(mut node: Rc<RefCell<Node>>) -> Vec<String> {
    std::iter::from_fn(|| {
        let parent = node.as_ref().borrow().parent.clone();
        match parent {
            Some(parent) => {
                node = parent.upgrade().unwrap();
                Some(node.as_ref().borrow().label.clone())
            }
            None => None,
        }
    })
    .collect()
}

pub fn calculate_distance(tree: &Tree, first_label: &str, second_label: &str) -> u32 {
    let first_path = get_path_to_root(tree.node_by_label.get(first_label).unwrap().clone());
    let second_path = get_path_to_root(tree.node_by_label.get(second_label).unwrap().clone());
    first_path.iter().rev().zip_longest(second_path.iter().rev()).map(|pair| {
        match pair {
            EitherOrBoth::Both(a, b) if a == b => 0,
            EitherOrBoth::Both(a, b) if a != b => 2,
            _ => 1,
        }
    }).sum()
}

pub fn parse_orbits<'a>(strings: impl Iterator<Item = &'a str>, root_label: &str) -> Tree {
    let mut objects: HashMap<String, _> = HashMap::new();

    for string in strings {
        let mut split_iter = string.split(')');
        let parent = split_iter.next().unwrap().to_string();
        let child = split_iter.next().unwrap().to_string();

        objects
            .entry(child.clone())
            .or_insert(Rc::new(RefCell::new(Node::new(&child))));

        let child_node = objects.get(&child).unwrap().clone();
        {
            let parent_entry = objects
                .entry(parent.clone())
                .or_insert(Rc::new(RefCell::new(Node::new(&parent))));

            parent_entry.borrow_mut().children.push(child_node);
        }
        let parent_node = objects.get(&parent).unwrap();
        objects.get_mut(&child).unwrap().borrow_mut().parent = Some(Rc::downgrade(parent_node));
    }

    Tree {
        root: objects.get(root_label).unwrap().clone(),
        node_by_label: objects,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let test_input = "COM)B\n\
                          B)C\n\
                          C)D\n\
                          D)E\n\
                          E)F\n\
                          B)G\n\
                          G)H\n\
                          D)I\n\
                          E)J\n\
                          J)K\n\
                          K)L";
        let tree = parse_orbits(test_input.split('\n'), "COM");
        assert_eq!(tree.root.as_ref().borrow().label, "COM");
        assert_eq!(calculate_sum_on_depths(&tree), 42);
    }

    #[test]
    fn second_test() {
        let test_input = "COM)B\n\
                          B)C\n\
                          C)D\n\
                          D)E\n\
                          E)F\n\
                          B)G\n\
                          G)H\n\
                          D)I\n\
                          E)J\n\
                          J)K\n\
                          K)L\n\
                          K)YOU\n\
                          I)SAN";
        let tree = parse_orbits(test_input.split('\n'), "COM");
        assert_eq!(calculate_distance(&tree, "YOU", "SAN"), 4);
    }
}
