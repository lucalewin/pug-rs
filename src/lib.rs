pub mod error;
pub mod prelude;
mod parser;

use crate::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::fs;
use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Arc<Mutex<Node>>>,
    pub id: Option<String>,
    pub element: String,
    pub class: Vec<String>,
    pub attrs: Vec<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            id: None,
            element: "".into(),
            class: Vec::new(),
            attrs: Vec::new(),
        }
    }
}

impl Node {
    pub fn special(name: String) -> Self {
        Self {
            children: Vec::new(),
            id: None,
            element: format!(":{name}"),
            class: Vec::new(),
            attrs: Vec::new(),
        }
    }

    pub fn text(bytes: &[u8]) -> Self {
        Self {
            children: Vec::new(),
            id: Some(String::from_utf8(bytes.to_vec()).unwrap()),
            element: ":text".into(),
            class: Vec::new(),
            attrs: Vec::new(),
        }
    }
}

impl From<&[u8]> for Node {
    fn from(b: &[u8]) -> Self {
        // println!("TODO: {b:?}");

        let mut index = 0;
        while index < b.len() && b[index].is_ascii_alphanumeric() {
            index += 1;
        }

        let name = &b[..index];

        Self {
            children: Vec::new(),
            id: None,
            element: String::from_utf8(name.to_vec()).unwrap(),
            class: Vec::new(),
            attrs: Vec::new(),
        }
    }
}

pub fn compile(file: impl AsRef<Path>) -> Result<Node> {
    let content = fs::read_to_string(file).expect("could not read file");

    let lines: Vec<&str> = content.lines().collect();

    let root = Arc::new(Mutex::new(Node::special("root".into())));

    {
        let mut stack: Vec<(isize, Arc<Mutex<Node>>)> = Vec::with_capacity(lines.len());

        stack.push((-1, root.clone()));

        for line in lines {
            let bytes = line.as_bytes();
            let indent = indent(bytes);

            if indent == bytes.len() {
                // empty line
                // ignore
                continue;
            }

            let node = match bytes[indent] {
                b'|' => {
                    // text node
                    Node::text(&bytes[(indent + 1)..])
                },
                b'{' => todo!("inline functions"),
                _ => {
                    // any other node
                    bytes[indent..].into()
                }
            };

            let node_arc_mutex = Arc::new(Mutex::new(node));

            while let Some((ind, n)) = stack.last_mut() {
                if *ind >= indent as isize {
                    // remove last element
                    // since the current indent is smaller
                    // then the indent of the last node on
                    // the stack, the last node on the stack
                    // won't have any more children
                    stack.pop();
                } else {
                    // current indent is bigger than the last
                    // node on the stack. That means the last
                    // element on the stack is the parent of
                    // the current node
                    n.lock().unwrap().children.push(node_arc_mutex.clone());
                    break;
                }
            }

            stack.push((indent as isize, node_arc_mutex));
        }
    }

    Ok(Arc::try_unwrap(root).unwrap().into_inner().unwrap())
}

pub fn render(n: Node) -> Result<()> {
    for child in n.children {

    }

    todo!()
}

fn indent(bytes: &[u8]) -> usize {
    let mut index = 0;
    while index < bytes.len() && bytes[index] == b' ' {
        index += 1;
    }
    index
}
