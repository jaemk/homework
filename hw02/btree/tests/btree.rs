extern crate btree;

use btree::{BST, Link};

#[test]
fn working() {
    assert_eq!(1, 1);
}

#[test]
fn create_new_empty_bst() {
    assert_eq!(BST::new().root, Link::Empty);
}

#[test]
fn add_to_empty() {
    let mut bst = BST::new();
    assert_eq!(bst.insert(3), true);
}

#[test]
fn add_multiple() {
    let mut bst = BST::new();
    assert_eq!(bst.insert(5), true);
    assert_eq!(bst.insert(3), true);
    assert_eq!(bst.insert(6), true);
    assert_eq!(bst.insert(1), true);
    assert_eq!(bst.insert(3), false);
}

#[test]
fn search_empty() {
    let bst = BST::new();
    assert_eq!(bst.search(3), false);
}

#[test]
fn search_populated() {
    let mut bst = BST::new();
    assert_eq!(bst.insert(4), true);
    assert_eq!(bst.insert(10), true);
    assert_eq!(bst.insert(20), true);
    assert_eq!(bst.insert(1), true);
    assert_eq!(bst.insert(3), true);
    assert_eq!(bst.insert(2), true);
    assert_eq!(bst.insert(5), true);
    assert_eq!(bst.insert(5), false);

    assert_eq!(bst.search(2), true);
    assert_eq!(bst.search(22), false);
}

#[test]
fn return_ordered_vec() {
    let mut bst = BST::new();
    assert_eq!(bst.insert(4), true);
    assert_eq!(bst.insert(10), true);
    assert_eq!(bst.insert(20), true);
    assert_eq!(bst.insert(1), true);
    assert_eq!(bst.insert(3), true);
    assert_eq!(bst.insert(2), true);
    assert_eq!(bst.insert(5), true);
    assert_eq!(bst.as_vec(), vec![1,2,3,4,5,10,20]);
}
