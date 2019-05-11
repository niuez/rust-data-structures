use std::cmp::max;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    size: usize,
    height: isize,
    child: [Link<T>; 2],
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node { elem: elem, size: 1, height: 1, child: [None, None] }
    }
}

fn size<T>(link: &Link<T>) -> usize {
    match link {
        &Some(ref node) => node.size,
        &None => 0,
    }
}

fn height<T>(link: &Link<T>) -> isize {
    match link {
        &Some(ref node) => node.height,
        &None => 0,
    }
}
fn diff_node<T>(node: &Box<Node<T>>) -> isize {
    height(&node.child[0]) - height(&node.child[1])
}

fn diff_link<T>(link: &Link<T>) -> isize {
    match link {
        &Some(ref node) => diff_node(node),
        &None => 0
    }
}

fn fix<T>(node: &mut Node<T>) {
    node.size = size(&node.child[0]) + size(&node.child[1]) + 1;
    node.height = max(height(&node.child[0]), height(&node.child[1])) + 1;
}

fn cut<T>(mut node: Box<Node<T>>, dir: usize) -> (Box<Node<T>>, Link<T>) {
    let nn = node.child[dir].take();
    fix(node.as_mut());
    (node, nn)
}

fn set<T>(mut node: Box<Node<T>>, dir_node: Link<T>, dir: usize) -> Box<Node<T>> {
    node.child[dir] = dir_node;
    fix(node.as_mut());
    node
}

fn rotate<T>(x: Box<Node<T>>, dir: usize) -> Box<Node<T>> {
    let (x, y) = cut(x, 1 - dir);
    let (y, B) = cut(y.unwrap(), dir);
    let x = set(x, B, 1 - dir);
    set(y, Some(x), dir)
}

fn balance<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    if diff_node(&node) == 2 {
        if diff_link(&node.child[0]) == -1 {
            let (n, ch) = cut(node, 0);
            node = set(n, Some(rotate(ch.unwrap(), 0)), 0);
        }
        rotate(node, 1)
    }
    else if diff_node(&node) == -2 {
        if diff_link(&node.child[1]) == 1 {
            let (n, ch) = cut(node, 1);
            node = set(n, Some(rotate(ch.unwrap(), 1)), 1);
        }
        rotate(node, 0)
    }
    else { node }
}

fn deepest_node<T>(mut node: Box<Node<T>>, dir: usize) -> (Box<Node<T>>, Link<T>) {
    let (mut n, ch) = cut(node, dir);
    match ch {
        Some(dir_node) => {
            let (deepest_node, dirn) = deepest_node(dir_node, dir);
            n = set(n, dirn, dir);
            (deepest_node, Some(n))
        }
        None => {
            cut(n, 1 - dir)
        }
    }
}

fn merge_dir<T>(dst: Box<Node<T>>, mut root: Box<Node<T>>, src: Link<T>, dir: usize) -> Box<Node<T>> {
    if (dst.height - height(&src)).abs() <= 1 {
        root = set(root, src, dir);
        root = set(root, Some(dst), 1 - dir);
        fix(&mut root);
        root
    }
    else {
        let (d, ch) = cut(dst, dir);
        match ch {
            Some(sch) => {
                let ch = Some(merge_dir(sch, root, src, dir));
                balance(set(d, ch, dir))
            }
            None => {
                balance(set(d, Some(balance(set(root, src, dir))), dir))
            }
        }
    }
}

fn merge<T>(mut left: Link<T>, mut right: Link<T>) -> Link<T> {
    match left {
        Some(ln) => {
            match right {
                Some(rn) => {
                    if ln.height >= rn.height {
                        let (deep_left, src) = deepest_node(rn, 0);
                        Some(merge_dir(ln, deep_left, src, 1))
                    }
                    else {
                        let (deep_right, src) = deepest_node(ln, 1);
                        Some(merge_dir(rn, deep_right, src, 0))
                    }
                }
                None => Some(ln),
            }
        }
        None => right,
    } 
}

fn at<T>(node: &Box<Node<T>>, i: usize) -> &T {
    assert!(diff_node(node).abs() <= 1);
    if size(&node.child[0]) == i {
        &node.elem
    }
    else if size(&node.child[0]) < i {
        at(node.child[1].as_ref().unwrap(), i - size(&node.child[0]) - 1)
    }
    else {
        at(node.child[0].as_ref().unwrap(), i)
    }
}

#[test]

fn merge_node_test() {
    let n0 = Box::new(Node::new(0));
    let n1 = Box::new(Node::new(1));
    let n2 = Box::new(Node::new(2));
    let n3 = Box::new(Node::new(3));
    let n4 = Box::new(Node::new(4));
    let n5 = Box::new(Node::new(5));
    let n0 = Some(n0);

    let n0 = merge(n0, Some(n1));
    println!("root = {}", n0.as_ref().unwrap().elem);
    let n0 = merge(n0, Some(n2));
    println!("root = {}", n0.as_ref().unwrap().elem);
    let n0 = merge(n0, Some(n3));
    println!("root = {}", n0.as_ref().unwrap().elem);
    let n0 = merge(n0, Some(n4));
    println!("root = {}", n0.as_ref().unwrap().elem);
    let n0 = merge(n0, Some(n5));
    println!("root = {}", n0.as_ref().unwrap().elem);
    println!("size = {}", n0.as_ref().unwrap().size);
    assert!(at(n0.as_ref().unwrap(), 0) == &0);
    assert!(at(n0.as_ref().unwrap(), 1) == &1);
    assert!(at(n0.as_ref().unwrap(), 2) == &2);
    assert!(at(n0.as_ref().unwrap(), 3) == &3);
    assert!(at(n0.as_ref().unwrap(), 4) == &4);
    assert!(at(n0.as_ref().unwrap(), 5) == &5);
}
