#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        let left = None;
        let right = None;

        Box::new(Node { left, right, value })
    }
}

fn main() {
    let mut root = Node::new(123);
    let left = Node::new(5);
    let right = Node::new(200);
    root.left = Some(left);
    root.right = Some(right);
    println!("{:?}", root);
}
