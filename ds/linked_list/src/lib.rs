//! This Library provides a Linked List struct
//! named `ListNode`, with some basic functions
//! implemented with the struct.
//!
//! Functions implemented:
//! * new -> `Self`
//! * from_vec -> `Self`
//! * print -> `()`
//! * push -> `()`
//! * push_back -> `()`
//! * delete -> `Result<(), &'static str>`
//! * find -> `Result<&Box<Self>, &'static str>`
//! * len -> `i32`
//! * reverse -> `()`
//! * copy -> `Box<ListNode<T>>`
//! * insert -> `()`
//! * pop -> `Option<T>`
//! * contains -> `bool`
//! * merge -> `Option<Box<ListNode<i32>>>`
//! * split_list -> `Vec<Option<Box<ListNode<i32>>>>`
//! * sort (merge) -> `Option<Box<ListNode<i32>>>`

use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::thread;

#[derive(Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    /// Constructs a new `ListNode<T>` with the one
    /// value provided
    ///
    /// # Example
    ///
    /// ```
    /// let linkedlist = ListNode::new(5);
    /// ```
    pub fn new(val: T) -> Box<Self> {
        Box::new(ListNode { val, next: None })
    }

    /// Constructs a `ListNode<T>` with `Vec<T>`
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 2, 3]);
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// 1 -> 2 -> 3 -> null
    /// ```
    pub fn from_vec(l: Vec<T>) -> Box<Self>
    where
        T: Clone,
    {
        if l.is_empty() {
            panic!("Vector can't be empty");
        }

        let mut list = ListNode::new(l[0].clone());
        let mut head = &mut list;
        for i in 1..l.len() {
            let newnode = ListNode::new(l[i].clone());
            head.next = Some(newnode);
            head = head.next.as_mut().unwrap();
        }
        list
    }

    /// Displays the linked list (`ListNode<T>`) into the format of
    /// ```
    /// val (address) -> val (address) ... -> null
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 2, 3]);
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// 1 -> 2 -> 3 -> null
    /// ```
    pub fn print(&self) -> ()
    where
        T: Display,
    {
        let mut head = Some(self);
        while let Some(node) = head {
            print!("{} ({:p}) -> ", node.val, node);
            head = node.next.as_deref();
        }
        print!("null");
        println!();
    }

    /// Appends an element to the beginning of the linked list
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::new(2);
    /// linkedlist.push(1);
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> null
    /// ```
    pub fn push(&mut self, val: T) -> ()
    where
        T: Clone,
    {
        let newnode = ListNode::new(val);
        let mut newhead = newnode;
        newhead.next = Some(Box::new(ListNode {
            val: self.val.clone(),
            next: self.next.take(),
        }));
        *self = *newhead;
    }

    ///Appends an element to the end of the linked list
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::new(1);
    /// linkedlist.push_back(2);
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> null
    /// ```
    pub fn push_back(&mut self, val: T) -> ()
    where
        T: Copy,
    {
        let mut head = self;
        loop {
            if head.next.is_none() {
                head.next = Some(ListNode::new(val));
                break;
            }
            head = head.next.as_deref_mut().unwrap();
        }
    }

    /// Delete the node with the given value `T`
    ///
    /// The function would create a dummy head and iterate
    /// through the entire list until the desired node is
    /// found. Once found, it would delete the node.
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 2, 2, 3]);
    /// linkedlist.delete(2);
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> 3 -> null
    /// ```
    pub fn delete(&mut self, val: T) -> Result<(), &'static str>
    where
        T: PartialEq,
    {
        let mut head = self;
        loop {
            if head
                .next
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("Node not found");
                })
                .val
                == val
            {
                let nextnode = head.next.as_mut().unwrap();
                if nextnode.val == val {
                    head.next = nextnode.next.take();
                    return Ok(());
                } else {
                    return Err("Node not found");
                }
            }
            head = head.next.as_deref_mut().unwrap();
        }
    }

    /// Finds the node with the a value `T`, and returns
    /// an instance of `Result<&Box<ListNode<T>>, &'static str>`
    ///
    /// # Example
    ///
    /// ```
    /// let mut listnode = ListNode::from_vec(vec![3, 1, 2]);
    /// let something = listnode.find(1);
    /// something.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2
    /// ```
    pub fn find(&mut self, val: T) -> Result<&Box<Self>, &'static str>
    where
        T: PartialEq,
    {
        let mut head = self;
        loop {
            if head
                .next
                .as_ref()
                .unwrap_or_else(|| {
                    panic!("Node not found");
                })
                .val
                == val
            {
                return Ok(head.next.as_ref().unwrap());
            }
            head = head.next.as_deref_mut().unwrap();
        }
    }

    /// Returns the length of the linked list
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 2, 3]);
    /// let length = linkedlist.len();
    /// print!(length);
    /// ```
    ///
    /// ## Output
    /// ```
    /// 3
    /// ```
    pub fn len(&mut self) -> i32 {
        let mut head = self;
        let mut count: i32 = 0;
        loop {
            if head.next.is_none() {
                count += 1;
                break;
            }
            count += 1;
            head = head.next.as_deref_mut().unwrap();
        }
        count
    }

    /// Reverses the given linked list
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![3, 2, 1]);
    /// linkedlist.reverse();
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> 3 -> null
    /// ```
    pub fn reverse(&mut self)
    where
        T: Copy,
    {
        let mut prev = None;
        let mut current = Some(Box::new(ListNode {
            val: std::mem::replace(&mut self.val, unsafe { std::mem::zeroed() }),
            next: self.next.take(),
        }));

        while let Some(mut boxed_node) = current {
            let next = boxed_node.next.take();
            boxed_node.next = prev;
            prev = Some(boxed_node);
            current = next;
        }

        if let Some(mut new_head) = prev {
            self.val = new_head.val;
            self.next = new_head.next.take();
        }
    }

    /// Produces a deep copy of a linked list
    ///
    /// # Example
    ///
    /// ```
    /// let linkedlist = ListNode::from_vec(vec![1, 2]);
    /// let another = linkedlist.copy();
    /// another.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> null
    /// ```
    pub fn copy(&self) -> Box<ListNode<T>>
    where
        T: Clone,
    {
        Box::new(ListNode {
            val: self.val.clone(),
            next: self.next.as_ref().map(|node| node.clone()),
        })
    }

    /// Inserts an element of type `T` into the linked list
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 3]);
    /// let _ = match linkedlist.insert(1, 2) { //(index: usize, element: T)
    ///     Ok(_) => (),
    ///     Err(e) => panic!("{e}"),
    /// `};
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    /// ```
    /// 1 -> 2 -> 3 -> null
    /// ````
    pub fn insert(&mut self, index: usize, val: T) -> Result<(), &str> {
        if index == 0 {
            let tempnode = Box::new(ListNode {
                val,
                next: self.next.take(),
            });
            self.next = Some(tempnode);
            std::mem::swap(&mut self.val, &mut self.next.as_mut().unwrap().val);
            return Ok(());
        }

        let mut head = self;
        for _ in 0..index - 1 {
            head = match head.next.as_mut() {
                Some(node) => node,
                None => return Err("Index out of range"),
            };
        }

        let tempnode = Box::new(ListNode {
            val,
            next: head.next.take(),
        });
        head.next = Some(tempnode);
        Ok(())
    }

    /// Returns the element at the top of the linked list
    ///
    /// This function returns the top element of the linked
    /// list as Option<T>, and also deletes the top element
    ///
    /// # Example
    ///
    /// ```
    /// let mut linkedlist = ListNode::from_vec(vec![1, 1, 2]);
    /// let poped = linkedlist.pop().unwrap();
    /// println!("{poped}");
    /// linkedlist.print();
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// 1
    /// 1 -> 2 -> null
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.next.is_none() {
            return Some(std::mem::replace(&mut self.val, unsafe {
                std::mem::zeroed()
            }));
        }

        let mut oldhead = self.next.take().unwrap();
        std::mem::swap(&mut self.val, &mut oldhead.val);
        self.next = oldhead.next.take();

        Some(oldhead.val)
    }

    /// Returns a bool of weather the linked list contains
    /// such element
    ///
    /// # Examlple
    ///
    /// ```
    /// let linkedlist = ListNode::from_vec(vec![1, 2, 3]);
    ///
    pub fn contains(&self, val: T) -> bool
    where
        T: PartialEq,
    {
        let mut head = self;
        while !head.next.is_none() {
            if head.val == val {
                return true;
            } else {
                head = head.next.as_deref().unwrap();
            }
        }
        return false;
    }

    /// Merges two sorted linked lists
    ///
    /// Takes two linked list with type `Option<Box<ListNode<i32>>>`
    /// and outputs a single list with the same type
    ///
    /// # Example
    ///
    /// ```
    /// let mut l1 = ListNode::from_vec(vec![1, 3]);
    /// let mut l2 = ListNode::from_vec(vec![2, 4]);
    /// let mut output = ListNode::merge(l1, l2);
    /// output.print();
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// 1 -> 2 -> 3 -> 4;
    /// ```
    pub fn merge(
        mut l1: Option<Box<ListNode<i32>>>,
        mut l2: Option<Box<ListNode<i32>>>,
    ) -> Option<Box<ListNode<i32>>>
    where
        T: PartialOrd,
    {
        let mut r = &mut l1;
        while l2.is_some() {
            if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut l2);
            }
            r = &mut r.as_mut()?.next;
        }
        l1
    }

    /// Splits the list into given parts
    ///
    /// # Example
    ///
    /// ```
    /// let mut list = ListNode::from_vec(vec![1, 2, 3, 4]);
    /// let (l1, l2) = {
    ///     let v = ListNode::split_list(Some(list), 2);
    ///     (
    ///         v[0].as_ref().unwrap().as_ref().clone,
    ///         v[1].as_ref().unwrap().as_ref().clone,
    ///     )
    /// }
    /// l1.print();
    /// l2.print();
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// l1 = 1 -> 2
    /// l2 = 3 -> 4
    /// ```
    pub fn split_list(head: Option<Box<ListNode<i32>>>, k: i32) -> Vec<Option<Box<ListNode<i32>>>> {
        let mut length = 0;
        let mut current = head.as_ref();
        let mut parts = Vec::new();

        while let Some(node) = current {
            length += 1;
            current = node.next.as_ref();
        }

        let (base_size, mut extra) = (length / k, length % k);
        let mut current = head;

        for _ in 0..k {
            let mut part_size = base_size + if extra > 0 { 1 } else { 0 };
            let mut dummy = Box::new(ListNode { val: 0, next: None });
            let mut tail = &mut dummy;

            while part_size > 0 {
                tail.next = current.take();
                tail = tail.next.as_mut().unwrap();
                current = tail.next.take();
                part_size -= 1;
            }

            parts.push(dummy.next.take());
            if extra > 0 {
                extra -= 1;
            }
        }

        parts
    }

    /// Sorts the linked list using merge sort and
    /// returns a new sorted linked list
    ///
    /// # Example
    ///
    /// ```
    /// let list ListNode::sort(Some(ListNode::from_vec(vec![1, 4, 2, 5, 3])));
    /// list.display();
    /// ```
    ///
    /// ## Output
    ///
    /// ```
    /// 1 -> 2 -> 3 -> 4 -> 5
    /// ```
    pub fn sort(head: Option<Box<ListNode<i32>>>) -> Option<Box<ListNode<i32>>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (left, right) = {
            let v = ListNode::<i32>::split_list(head, 2);
            (
                v[0].as_ref().unwrap().as_ref().clone(),
                v[1].as_ref().unwrap().as_ref().clone(),
            )
        };

        let left_handle = thread::spawn(move || ListNode::<i32>::sort(Some(Box::new(left))));
        let right_handle = thread::spawn(move || ListNode::<i32>::sort(Some(Box::new(right))));

        let left_sorted = left_handle.join().unwrap();
        let right_sorted = right_handle.join().unwrap();

        ListNode::<i32>::merge(left_sorted, right_sorted)
    }
}

impl<T> Index<usize> for ListNode<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut head = self;

        for _ in 0..index {
            head = head.next.as_deref().expect("Index out of range");
        }
        &head.val
    }
}

impl<T> IndexMut<usize> for ListNode<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut head = self;

        for _ in 0..index {
            head = head.next.as_deref_mut().expect("Index out of range");
        }
        &mut head.val
    }
}
