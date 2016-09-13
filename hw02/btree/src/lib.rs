use std::mem;

// handle of binary-search-tree
#[derive(Debug)]
pub struct BST {
    pub root: Link,
    size: usize,
}

// branch to node
#[derive(Debug, PartialEq)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

// populated node
#[derive(Debug, PartialEq)]
pub struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl Link {
    /// Create a new Link with a populated Node containing
    /// the given elem and empty branches.
    fn new(n: i32) -> Link {
        Link::More(
            Box::new(
                Node{elem: n,
                     left: Link::Empty,
                     right: Link::Empty,
                    }
            )
        )
    }
}

/// Traverse tree to appropriate node and insert if
/// node is not already popualated.
fn tree_insert(link: &mut Link, item: i32) -> bool {
    match link {
        &mut Link::Empty => {
            let newlink = Link::new(item);
            mem::replace(link, newlink);
            true
        }
        &mut Link::More(ref mut node) => {
            if item == node.elem {
                false
            } else if item < node.elem {
                tree_insert(&mut node.left, item)
            } else {
                tree_insert(&mut node.right, item)
            }
        }
    }
}

/// Traverse tree until item or an empty link is found
fn tree_search(link: &Link, item: i32) -> bool {
    match link {
        &Link::Empty => false,
        &Link::More(ref node) => {
            if item == node.elem {
                true
            } else if item < node.elem {
                tree_search(&node.left, item)
            } else {
                tree_search(&node.right, item)
            }
        }
    }
}

/// Collect tree nodes in ascending order into a vec
fn tree_collect(mut vec: &mut Vec<i32>, link: &Link) {
    match link {
        &Link::More(ref node) => {
            if node.left != Link::Empty {
                tree_collect(&mut vec, &node.left);
            }
            vec.push(node.elem);
            if node.right != Link::Empty {
                tree_collect(&mut vec, &node.right);
            }
        }
        _ => (),
    };
}

impl BST {
    pub fn new() -> BST {
        BST{root: Link::Empty, size: 0}
    }
    pub fn insert(&mut self, item: i32) -> bool {
        self.size += 1;
        if self.root == Link::Empty {
            mem::replace(&mut self.root, Link::new(item));
            true
        } else {
            tree_insert(&mut self.root, item)
        }
    }
    pub fn search(&self, item: i32) -> bool {
        if self.root == Link::Empty {
            false
        } else {
            tree_search(&self.root, item)
        }
    }
    pub fn as_vec(&self) -> Vec<i32> {
        let mut vec = Vec::with_capacity(self.size);
        tree_collect(&mut vec, &self.root);
        vec
    }
}
