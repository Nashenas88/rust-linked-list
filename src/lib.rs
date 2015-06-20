#![feature(box_syntax, box_patterns)]
#![allow(dead_code)]
use std::mem;
use std::ops::Index;

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            length: 0,
        }
    }

    fn push(&mut self, value: T) {
        Node::push(&mut self.head, value);
        self.length += 1;
    }

    fn push_front(&mut self, value: T) {
        let mut head = Node::new(value);
        head.next = self.head.take();
        self.head = Some(box head);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        (match index {
            // special case for head
            0 => self.head.take().map(|mut node| {
                self.head = node.next.take();
                node.value
            }),
            // grab at index - 1
            _ => Self::_index_mut(&mut self.head, index - 1)
                .and_then(|mut prev_link| prev_link.as_mut()
                    .and_then(|mut prev_node| prev_node.next.take()
                        .map(|box node| {
                            prev_node.next = node.next;
                            node.value
                        }))),
        }).map(|ret| {
            self.length -= 1;
            ret
        })
    }

    fn _index_mut(prev: &mut Link<T>, _index: usize) -> Option<&mut Link<T>> {
        match _index {
            0 => Some(prev),
            _ => prev.as_mut()
                .and_then(|mut node| Self::_index_mut(&mut node.next, _index - 1)),
        }
    }
}

/*impl<T> LinkedList<T> where T: PartialEq {
    fn remove_value(&mut self, value: T) -> Option<T> {
        let mut next: Link<T> = None;
        let result = match self.head {
            None => Some(None),
            Some(ref mut node) => if node.value == value {
                next = node.next.take();
                Some(Some(node.value))
            } else {
                None
            }
        };
        
        match result {
            None => (),
            Some(ret) => {
                self.head = next;
                return ret;
            }
        }
        
        Self::_by_value_mut(&mut self.head, value)
            .and_then(|mut prev_link| prev_link.as_mut()
                .and_then(|mut prev_node| prev_node.next.take()
                    .map(|box node| {
                        prev_node.next = node.next;
                        node.value
                    })))
            .map(|ret| {
                self.length -= 1;
                ret
            })
    }

    fn _by_value_mut(prev: &mut Link<T>, value: T) -> Option<&mut Link<T>> {
        prev.as_mut().and_then(|mut prev_node| {
            prev_node.next.and_then(|node| if node.value == value {
                Some(prev)
            } else {
                Self::_by_value_mut(&mut prev_node.next, value)
            })
        })
    }
}*/

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    
    fn index(&self, _index: usize) -> &T {
        match self.head {
            Some(ref node) => &node[_index],
            None => panic!("Access out of bounds"),
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: value,
            next: None,
        }
    }

    fn push(node: &mut Link<T>, value: T) {
        if let Some(box ref mut n) = *node {
            Self::push(&mut n.next, value);
        } else {
            mem::replace(node, Some(box Node::new(value)));
        }
    }
}

impl<T> Index<usize> for Node<T> {
    type Output = T;

    fn index(&self, _index: usize) -> &T {
        if _index == 0 {
            &self.value
        } else if let Some(ref node) = self.next {
            node.index(_index - 1)
        } else {
            panic!("Access out of bounds");
        }
    }
}

#[test]
fn test_constructor() {
    let linked_list = LinkedList::<usize>::new();
    assert!(linked_list.head.is_none());
    assert_eq!(linked_list.length, 0);
}

#[test]
fn test_push() {
    let mut linked_list = LinkedList::new();
    linked_list.push(1);
    linked_list.push(2);
    linked_list.push(3);

    assert_eq!(linked_list[0], 1);
    assert_eq!(linked_list[1], 2);
    assert_eq!(linked_list[2], 3);
}

#[test]
fn test_remove() {
    let mut linked_list = LinkedList::new();
    linked_list.push(1);
    linked_list.push(2);
    linked_list.push(3);
    linked_list.push(4);

    assert_eq!(linked_list.remove(1).expect("Should have returned a value"), 2);
    assert_eq!(linked_list[0], 1);
    assert_eq!(linked_list[1], 3);
    assert_eq!(linked_list[2], 4);

    assert_eq!(linked_list.remove(0).expect("Should have returned a value"), 1);
    assert_eq!(linked_list[0], 3);
    assert_eq!(linked_list[1], 4);
    
    assert_eq!(linked_list.remove(1).expect("Should have returned a value"), 4);
    assert_eq!(linked_list[0], 3);
    
    assert!(linked_list.remove(1).is_none());
    assert_eq!(linked_list[0], 3);

    assert_eq!(linked_list.remove(0).expect("Should have returned a value"), 3);
    assert_eq!(linked_list.length, 0);
    
    assert!(linked_list.remove(0).is_none());
    assert_eq!(linked_list.length, 0);
}
