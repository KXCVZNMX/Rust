//! This crate provides some data structures
//! implemented by [KXCVZNMX](https://github.com/KXCVZNMX)
//!
//! Data structures currently implemented:
//! * [Singly Linked-List](linked_list)

pub mod ds {
    /// This module provides a Slngly Linked List struct
    /// named `ListNode`
    ///
    /// Functions implemented:
    /// * [new](struct.ListNode.html#method.new) -> `Box<Self>`
    /// * [from_vec](struct.ListNode.html#method.from_vec) -> `Box<Self>`
    /// * [print](struct.ListNode.html#method.print) -> `()`
    /// * [push](struct.ListNode.html#method.push) -> `()`
    /// * [push_back](struct.ListNode.html#method.push_back) -> `()`
    /// * [delete](struct.ListNode.html#method.delete) -> `Result<(), &'static str>`
    /// * [find](struct.ListNode.html#method.find) -> `Result<&Box<Self>, &'static str>`
    /// * [len](struct.ListNode.html#method.len) -> `i32`
    /// * [reverse](struct.ListNode.html#method.reverse) -> `()`
    /// * [copy](struct.ListNode.html#method.copy) -> `Box<ListNode<T>>`
    /// * [insert](struct.ListNode.html#method.insert) -> `()`
    /// * [pop](struct.ListNode.html#method.pop) -> `Option<T>`
    /// * [contains](struct.ListNode.html#method.contains) -> `bool`
    /// * [merge](struct.ListNode.html#method.merge) -> `Option<Box<ListNode<i32>>>`
    /// * [sort](struct.ListNode.html#method.sort) -> `Option<Box<ListNode<i32>>>`
    pub mod linked_list {
        use std::fmt::Display;
        use std::ops::{Index, IndexMut};
        use std::thread;

        #[derive(Clone, Debug)]
        pub struct ListNode<T> {
            pub val: T,
            pub next: Option<Box<ListNode<T>>>,
        }

        impl<T> ListNode<T> {
            /// Constructs a new instance of `ListNode<T>` with
            /// the provided `val: T`, returning `Box<ListNode<T>>`
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let linked_list = ListNode::new(0);
            /// assert_eq!(linked_list, Box::new( ListNode{ val: 0, next: None } ));
            /// ```
            ///
            /// Variable `linked_list` will have a value of `0`
            /// and next pointing to `None`:
            pub fn new(val: T) -> Box<Self> {
                Box::new(ListNode { val, next: None })
            }

            /// Constructs a new instance of `ListNode<T>` with
            /// a `Vec<T>`, returning `Box<ListNode<T>>`
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// assert_eq!(
            ///     ListNode::from_vec(vec![1, 2, 3]),
            ///     Box::new(ListNode {
            ///         val: 1,
            ///         next: Some(Box::new(ListNode {
            ///             val: 2,
            ///             next: Some(Box::new(ListNode {
            ///                 val: 3,
            ///                 next: None,
            ///             }))
            ///         }))
            ///     })
            /// );
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

            /// Prints the provided `ListNode<T>`
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let list = ListNode::from_vec(vec![1, 2, 3]);
            /// list.print(); //Output = 1 -> 2 -> 3 -> None
            pub fn print(&self) -> ()
            where
                T: Display,
            {
                let mut head = Some(self);
                while let Some(node) = head {
                    print!("{} ({:p}) -> ", node.val, node);
                    head = node.next.as_deref();
                }
                print!("None");
                println!();
            }

            /// Pushes an instance of `ListNode<T>` to the front of the list
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let mut list = ListNode::from_vec(vec![2, 3]);
            /// list.push(1);
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3]));
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

            /// Pushes an instance of `ListNode<T>` to the back of the list
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let mut list = ListNode::from_vec(vec![1, 2]);
            /// list.push_back(3);
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3]));
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

            /// Deletes the node that equals to the given `val: T`
            ///
            /// The function will delete the first node in the sequence
            /// iterating from `head`, not by index.
            ///
            /// If the element is not found, the program would panic,
            /// yielding `Node not found`.
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo() -> Result<(), &'static str> {
            /// let mut list = ListNode::from_vec(vec![1, 2, 2, 3]);
            /// list.delete(2)?;
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3]));
            /// # Ok(())
            /// # }
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
                        }
                        return Err("Node not found");
                    }
                    head = head.next.as_deref_mut().unwrap();
                }
            }

            /// Finds the node with the give `val: T` and
            /// returns a reference of element.
            ///
            /// This function does not return a new copy of the linked-list, it
            /// returns a reference to a node already on the list.
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo<T: Default>() -> Result<Box<ListNode<T>>, &'static str> {
            ///  let mut list = ListNode::from_vec(vec![1, 2, 3]);
            ///  let found_node = list.find(2)?;
            ///  assert_eq!(found_node.val, 2);
            ///  // 1 (list's head) -> 2 (found_node's head) -> 3;
            ///  // find() does not copy the original list
            ///  // so the two pointers could potentially
            ///  // be the same element
            ///  let mut list2 = ListNode::from_vec(vec![1, 2, 3]);
            ///  {
            ///     let found_node2 = list2.find(1)?;
            ///     assert_eq!(found_node2.val, 1);
            ///  }
            ///  // compares the pointers
            ///  assert_eq!(
            ///     format!("{:p}", list2.as_ref()),
            ///     format!("{:p}", list2.find(1)?.as_ref())
            /// );
            ///  // 1 (list2's head and found_node2's head) -> 2 -> 3
            ///  // you could print the list as well, as the
            ///  // print function prints the pointer address too
            /// # Ok(Box::new(*ListNode::new(T::default())))
            /// # }
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

            /// Return the length of the given `ListNode<T>` as an i32
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let mut list = ListNode::from_vec(vec![1, 2, 3]);
            /// assert_eq!(list.len(), 3);
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

            /// Reverses the given `ListNode<T>`
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let mut list = ListNode::from_vec(vec![1, 2, 3]);
            /// let mut rev_list = ListNode::from_vec(vec![3, 2, 1]);
            /// rev_list.reverse();
            /// assert_eq!(list, rev_list);
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

            /// Deep copies the given `ListNode<T>`
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// let list = ListNode::from_vec(vec![1, 2, 3]);
            /// let copied = list.copy();
            /// assert_eq!(list, copied);
            /// // compares the pointers
            /// assert_ne!(
            ///     (format!(
            ///         "{:p}{:p}{:p}",
            ///         list,
            ///         list.next.as_ref().unwrap(),
            ///         list.next.as_ref().unwrap().next.as_ref().unwrap())),
            ///     (format!(
            ///         "{:p}{:p}{:p}",
            ///         copied,
            ///         copied.next.as_ref().unwrap(),
            ///         copied.next.as_ref().unwrap().next.as_ref().unwrap())),
            /// );
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

            /// Inserts a new node of `ListNode<T>` with the `val: T` and in position `index: usize`
            ///
            /// if the index is out of range, the function would return Err
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo() -> Result<(), &'static str> {
            ///  let mut list = ListNode::from_vec(vec![1, 3]);
            ///  list.insert(1, 2)?;
            ///  assert_eq!(list, ListNode::from_vec(vec![1, 2, 3]));
            /// # Ok(())
            /// # }
            /// ```
            pub fn insert(&mut self, index: usize, val: T) -> Result<(), &'static str> {
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

            /// Pops the element on the front of the list
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo<T>() -> Option<T> {
            /// let mut list = ListNode::from_vec(vec![1, 1, 2, 3]);
            /// list.pop()?;
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3]));
            /// # None
            /// # }
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

            /// Checks if whether the `ListNode<T>` contains the given `val: T` element
            ///
            /// # Example
            /// ```
            /// # use data_structure::ds::linked_list::ListNode;
            /// let list = ListNode::from_vec(vec![1, 2, 3]);
            /// assert_eq!(list.contains(2), true);
            /// ```
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
                false
            }

            /// Merges two sorted `ListNode<T>` while keeping the order
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo() -> Option<Box<ListNode<i32>>> {
            /// let temp1 = ListNode::from_vec(vec![1, 3]);
            /// let temp2 = ListNode::from_vec(vec![2, 4, 5]);
            /// let list = ListNode::<i32>::merge(Some(temp1), Some(temp2))?;
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3, 4, 5]));
            /// # Some(ListNode::new(0))
            /// # }
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

            fn split_list(head: Option<Box<ListNode<i32>>>, k: i32) -> Vec<Option<Box<ListNode<i32>>>> {
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

            /// Sorts the `ListNode<T>` through merge sort
            ///
            /// # Example
            /// ```
            /// # use crate::data_structure::ds::linked_list::ListNode;
            /// # fn foo() -> Option<Box<ListNode<i32>>> {
            /// let list = ListNode::<i32>::sort(Some(ListNode::from_vec(vec![5, 2, 3, 1, 4])))?;
            /// assert_eq!(list, ListNode::from_vec(vec![1, 2, 3, 4, 5]));
            /// # Some(ListNode::new(0))
            /// # }
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

        impl<T: PartialEq> PartialEq for ListNode<T> {
            fn eq(&self, other: &Self) -> bool {
                let mut head = Some(self);
                let mut other_head = Some(other);
                while let (Some(current), Some(other_current)) = (head, other_head) {
                    if current.val != other_current.val {
                        return false;
                    }
                    head = current.next.as_deref();
                    other_head = other_current.next.as_deref();
                }
                head.is_none() && other_head.is_none()
            }
        }
    }

    pub mod bst {
        use std::rc::Rc;
        use std::cell::RefCell;

        #[derive(Debug, Clone)]
        pub struct BST<T> {
            pub val: T,
            pub left: Option<Rc<RefCell<BST<T>>>>,
            pub right: Option<Rc<RefCell<BST<T>>>>
        }

        impl<T> BST<T> {
            pub fn new(val: T, left: Option<Rc<RefCell<BST<T>>>>, right: Option<Rc<RefCell<BST<T>>>>) -> Rc<RefCell<BST<T>>> {
                Rc::new(RefCell::new(Self {
                    val,
                    left,
                    right,
                }))
            }

            pub fn from_vec(v: &Vec<T>) -> Result<Option<Rc<RefCell<BST<T>>>>, &'static str>
            where T:
                Clone + Ord
            {

                let mid = v.len() / 2;
                let root = BST::new(v[mid].clone(), None, None);

                if v.len() > 2 {
                    root.borrow_mut().left = BST::from_vec(&v[..mid].to_vec())?;
                    root.borrow_mut().right = BST::from_vec(&v[mid + 1..].to_vec())?;
                }

                Ok(Some(root))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use crate::ds::bst::BST;
    use crate::ds::linked_list::ListNode;
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut l1 = ListNode::new(1);
        let temp = ListNode::from_vec(vec![2, 3, 4, 5]);
        l1.next = Some(temp);
        assert_eq!(l1, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l2 = ListNode::from_vec(vec![2, 3, 4, 5]);
        l2.push(1);
        assert_eq!(l2, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l4 = ListNode::from_vec(vec![1, 2, 3, 4]);
        l4.push_back(5);
        assert_eq!(l4, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l5 = ListNode::from_vec(vec![1, 2, 2, 3, 4, 5]);
        match l5.delete(2) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }
        assert_eq!(l5, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l6 = ListNode::from_vec(vec![3, 3, 3, 1, 2, 3, 4, 5]);
        let l6 = match l6.find(1) {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(l6, &ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l7 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(l7.len(), 5);

        let mut l8 = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        l8.reverse();
        assert_eq!(l8, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let temp = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let l9 = temp.copy();
        assert_eq!(temp, l9);

        let l10 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let element = l10[1];
        let element2 = l10[2];
        assert_eq!(element, 2);
        assert_eq!(element2, 3);

        let mut l11 = ListNode::from_vec(vec![1, 3, 4, 5]);
        let _ = match l11.insert(1, 2) {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        };
        assert_eq!(l11, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let mut l12 = ListNode::from_vec(vec![1, 1, 2, 3, 4, 5]);
        let _poped = l12.pop().unwrap();
        assert_eq!(l12, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let l13 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let found = l13.contains(3);
        assert_eq!(found, true);
        let notfound = l13.contains(0);
        assert_eq!(notfound, false);

        let t1 = ListNode::from_vec(vec![1, 3, 5]);
        let t2 = ListNode::from_vec(vec![2, 4]);
        let l16 = ListNode::<i32>::merge(Some(t1), Some(t2)).unwrap();
        assert_eq!(l16, ListNode::from_vec(vec![1, 2, 3, 4, 5]));

        let l18 = ListNode::from_vec(vec![1, 5, 3, 2, 4]);
        let l18 = ListNode::<i32>::sort(Some(l18)).unwrap();
        assert_eq!(l18, ListNode::from_vec(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_bst() {
        let b1 = BST::new(
            4,
            Some(BST::new(2, Some(BST::new(1, None, None)), Some(BST::new(3, None, None)))),
            Some(BST::new(6, Some(BST::new(5, None, None)), Some(BST::new(7, None, None)))),
        );
        println!("{} {} {} {} {} {} {}",
                 b1.borrow().deref().val,
                 b1.borrow().deref().clone().left.unwrap().borrow().val,
                 b1.borrow().deref().clone().left.unwrap().borrow().left.clone().unwrap().borrow().val,
                 b1.borrow().deref().clone().left.unwrap().borrow().right.clone().unwrap().borrow().val,
                 b1.borrow().deref().clone().right.unwrap().borrow().val,
                 b1.borrow().deref().clone().right.unwrap().borrow().left.clone().unwrap().borrow().val,
                 b1.borrow().deref().clone().right.unwrap().borrow().right.clone().unwrap().borrow().val,
        );
    }
}