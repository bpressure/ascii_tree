extern crate ascii_tree;
use ascii_tree::write_tree;
use ascii_tree::Tree::*;

#[test]
fn test_tree() {
    let l1 = Leaf(vec![String::from("line1"), String::from("line2"), String::from("line3"), String::from("line4")]);
    let l2 = Leaf(vec![String::from("only one line")]);
    let n1 = Node(String::from("node 1"), vec![l1.clone(), l2.clone()]);
    let n2 = Node(String::from("node 2"), vec![l2.clone(), l1.clone(), l2.clone()]);
    let n3 = Node(String::from("node 3"), vec![n1.clone(), l1.clone(), l2.clone()]);
    let n4 = Node(String::from("node 4"), vec![n1, n2, n3]);

    let mut output = String::new();
    let _ = write_tree(&mut output, &n4);
    assert_eq!(String::from(" node 4
 ├─ node 1
 │  ├─ line1
 │  │  line2
 │  │  line3
 │  │  line4
 │  └─ only one line
 ├─ node 2
 │  ├─ only one line
 │  ├─ line1
 │  │  line2
 │  │  line3
 │  │  line4
 │  └─ only one line
 └─ node 3
    ├─ node 1
    │  ├─ line1
    │  │  line2
    │  │  line3
    │  │  line4
    │  └─ only one line
    ├─ line1
    │  line2
    │  line3
    │  line4
    └─ only one line
"),output);
}