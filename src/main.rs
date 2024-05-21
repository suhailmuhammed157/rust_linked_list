#[derive(Debug)]

struct LinkedList {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Option<Box<Node>>,
}

// implement drop trait
impl Drop for LinkedList {
    fn drop(&mut self) {
        println!(
            "dropping the linked list {:?}",
            &self.head.as_ref().unwrap()
        );
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // add a new node at the head and make the current head as the next of new node
    fn add_node(&mut self, element: i32) {
        let current_head = self.head.take();
        let new_head = Node {
            element,
            next: current_head,
        };
        self.head = Some(Box::new(new_head));
    }

    // remove the node from the current head and make the next node of the removed head as the new head
    fn remove_node(&mut self) -> Option<i32> {
        let current_head = self.head.take().unwrap();
        self.head = current_head.next;
        Some(current_head.element)
    }

    // print the element of all the node from the list
    fn print_list(&self) {
        let mut node_traversal = &self.head;

        while !node_traversal.is_none() {
            println!("{:?}", node_traversal.as_ref().unwrap().element);
            node_traversal = &node_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.add_node(1);
    linked_list.add_node(2);
    linked_list.add_node(3);
    linked_list.add_node(4);
    linked_list.add_node(5);
    linked_list.remove_node();

    linked_list.print_list();
}
