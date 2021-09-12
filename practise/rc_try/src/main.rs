use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v|v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());

    // let node5 = Node::new(5);
    // let mut node3 = node1.get_downstream().unwrap();
    // node3.update_downstream(Rc::new(node5));
    // println!("node1: {:#?}, node2: {:#?}", node1, node2);
}

#[test]
fn test_rc_strong_count() {
    let a = Rc::new(3);
    assert_eq!(Rc::strong_count(&a), 1);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    assert_eq!(Rc::strong_count(&a), 3);
    assert_eq!(Rc::strong_count(&b), 3);
    assert_eq!(Rc::strong_count(&c), 3);
}

#[test]
fn test_ref_cell() {
    let data = RefCell::new(1);
    // 若不添加 {} 程序会运行到 64 行崩溃
    // 原因： already mutably borrowed
    // 重要：所有权的借用规则在此依旧有效，只不过它在运行时检测 
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data: {:?}", data.borrow());
}