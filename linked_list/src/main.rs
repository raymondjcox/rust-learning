struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Node>,
}

impl LinkedList {
    fn add(&self, value: i32) -> Node {
        let next = self.head;
        let node = Node { value, next };
        self.head = node;
        self.head
    }
}

fn main() {
    println!("Hello, world!");
    let list = LinkedList { head: None };
    let cur = list.head;

    while cur.next != None {
        println!("{}", cur.value);
        cur = cur.next;
    }
}
