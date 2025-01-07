use std::collections::LinkedList;

//LinkedList
// Detect Cycle
//  Merge Two Sorted Lists
//  Remove Nth Node From End
// Find Middle of the Linked List
// LinkedList Cycle II (find cycle start)
// Add Two Numbers (represented as LinkedLists)
// Intersection of Two LinkedLists
//  Palindrome LinkedList

//create a linked list with display and push methods
// pub struct Node<T> {
//     pub value: T,
//     pub next: Option<Box<Node<T>>>,
// }

// pub struct LinkedList<T> {
//     pub head: Option<Box<Node<T>>>,
// }

// impl<T> LinkedList<T> {
//     pub fn new() -> Self {
//         LinkedList { head: None }
//     }

//     pub fn push(&mut self, value: T) {
//         let new_node = Box::new(Node {
//             value,
//             next: self.head.take(),
//         });
//         self.head = Some(new_node);
//     }

//     pub fn display(&self)
//     where
//         T: std::fmt::Debug,
//     {
//         let mut current = &self.head;
//         while let Some(node) = current {
//             print!("{:?} -> ", node.value);
//             current = &node.next;
//         }
//         println!("None");
//     }
// }



//Given a linked list, the task is to reverse the linked list by changing the links between nodes.
pub fn reverseLinkedList()
{
    let mut linked_list: LinkedList<char> = LinkedList::new();
    linked_list.push_back('a');
    linked_list.push_back('b');
    linked_list.push_back('c');    
    println!("Original LinkedList: {:?}", linked_list);
    let mut reversed_list: LinkedList<char> = LinkedList::new();
    
    while let Some(value) = linked_list.pop_back() {
        reversed_list.push_back(value);
    }
    
    println!("Reversed LinkedList: {:?}", reversed_list);


    // while let Some(node) = current_node {
    //     println!(
    //         "Node {}: {:?}, Next: {:?}",
    //         index,
    //         node,
    //         linked_list.iter().skip(index + 1).next()
    //     );
    //     current_node = linked_list.iter().skip(index + 1).next();
    //     index += 1;
    // }
}


//cycled linked list
pub fn detect_cycle(linked_list: &LinkedList<char>) -> bool {
    let mut slow = linked_list.front();  // Tortoise (moves one step at a time)
    let mut fast = linked_list.front();  // Hare (moves two steps at a time)

    while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
        // Move slow by one step
        slow = linked_list.iter().skip_while(|&node| node != slow_node).nth(1);
        
        // Move fast by two steps
        fast = linked_list.iter().skip_while(|&node| node != fast_node).nth(2);

        // If fast and slow meet, a cycle exists
        if slow == fast {
            return true;
        }
    }

    // No cycle detected
    false
}

// pub fn create_linked_list_with_cycle() -> LinkedList<char> {
//     let mut linked_list: LinkedList<char> = LinkedList::new();
//     linked_list.push_back('a');
//     linked_list.push_back('b');
//     linked_list.push_back('c');
    
//     // Create a cycle manually for testing
//     let last_node = linked_list.back_mut().unwrap();
//     let second_node = linked_list.iter_mut().nth(1).unwrap();
//     last_node.next= Some(second_node);

//     linked_list
// }


//Merge Two Sorted Lists
