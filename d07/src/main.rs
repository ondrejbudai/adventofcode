extern crate core;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::rc::Rc;

type ChildrenMap = HashMap<String, Rc<RefCell<Node>>>;

enum NodeValue {
    Children(ChildrenMap),
    LeafValue(usize),
}

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    value: NodeValue,
}

impl Node {
    fn sum(&self) -> (usize, Vec<usize>) {
        let mut sum: usize = 0;
        let mut childen_sizes: Vec<usize> = Vec::new();
        match &self.value {
            NodeValue::Children(map) => {
                for (_, child) in map {
                    let (own_size, mut subtree_sizes) = child.borrow().sum();
                    sum += own_size;
                    childen_sizes.append(&mut subtree_sizes);
                }
            }
            NodeValue::LeafValue(size) => {
                return (*size, Vec::new())
            }
        }
        childen_sizes.push(sum);
        return (sum, childen_sizes);
    }
}


fn main() {
    let root = Rc::new(RefCell::new(Node { parent: None, value: NodeValue::Children(HashMap::new()) }));
    let mut cwd = root.clone();

    stdin().lock().lines().for_each(|x| {
        let line = x.unwrap();
        if line.starts_with("$") {
            let argv = &line[2..].split(" ").collect::<Vec<&str>>();
            let cmd = argv.get(0).unwrap();
            if *cmd == "cd" {
                let mut new_cwd = cwd.clone();
                let dest = argv.get(1).unwrap();
                if *dest == ".." {
                    new_cwd = cwd.borrow().parent.clone().unwrap().clone();
                } else if *dest == "/" {
                    new_cwd = root.clone();
                } else {
                    let children_or_leaf = &mut cwd.borrow_mut().value;

                    if let NodeValue::Children(children) = children_or_leaf {
                        let child = children.get(*dest).unwrap();
                        new_cwd = child.clone();
                    }
                }

                cwd = new_cwd;
            }
        } else {
            let mut splitted_line = line.split(" ");
            let size_or_dir = splitted_line.next().unwrap();
            let filename = splitted_line.next().unwrap();
            let value = &mut cwd.borrow_mut().value;
            if let NodeValue::Children(children) = value {
                if size_or_dir == "dir" {
                    children.insert(filename.to_string(), Rc::new(RefCell::new(Node { parent: Some(cwd.clone()), value: NodeValue::Children(HashMap::new()) })));
                } else {
                    let size = size_or_dir.parse::<usize>().unwrap();
                    children.insert(filename.to_string(), Rc::new(RefCell::new(Node { parent: Some(cwd.clone()), value: NodeValue::LeafValue(size) })));
                }
            }
        }
    });

    let (_, mut subtree_sizes) = root.borrow().sum();
    let mut sum: usize = 0;
    subtree_sizes.iter().for_each(|size| {
        if *size <= 100000 {
            sum += *size;
        }
    });

    println!("{}", sum);

    let need_freespace = 30000000 - (70000000 - subtree_sizes.last().unwrap());
    let mut directory_to_delete = 0;
    subtree_sizes.sort();

    for size in subtree_sizes {
        directory_to_delete = size;
        if size > need_freespace {
            break;
        }
    }

    println!("{}", directory_to_delete);
}
