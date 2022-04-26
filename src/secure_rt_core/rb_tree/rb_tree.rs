extern crate alloc;

use super::rb_node::*;
use core::cell::RefCell;
use alloc::rc::Rc;
use core::marker::Copy;

pub struct RBTree<K, V> {
    root: Option<Rc<RefCell<RBNode<K, V>>>>,
}

impl<K, V> RBTree<K, V> 
        where K: Clone + PartialEq + PartialOrd + Copy, V: Clone {
    pub fn new() -> Self {
        return Self { root: None };
    }
    fn rotate_left(&mut self, this: Rc<RefCell<RBNode<K, V>>>) {
        if this.borrow().r_child.clone().is_none() {
            // unsure how to handle this case
            return;
        } 
        let p_node = this.borrow().parent.clone();
        // r_child is safe to unwrap
        let r_child = this.borrow().r_child.as_ref().unwrap().clone();
        this.borrow_mut().color_swap(r_child.borrow_mut());
        // unsure if l_child is Some... clone the option
        this.borrow_mut().r_child = r_child.borrow().l_child.clone();
        r_child.borrow_mut().l_child = Some(this.clone());

        // unsure if parent is Some... clone the option
        r_child.borrow_mut().parent = this.borrow().parent.clone();
        this.borrow_mut().parent = Some(r_child.clone());

        if p_node.is_none() {
            self.root = Some(r_child.clone());
            return;
        }
        // parent is safe to unwrap
        let parent = p_node.unwrap();
        let mut parent = parent.borrow_mut();
        // is parent l_child safe to unwrap?
        if let Some(parent_l_child) = &parent.l_child {
            if this.borrow().key == parent_l_child.borrow().key {
                parent.l_child = Some(r_child.clone());
                return;
            }
        }
        parent.r_child = Some(r_child.clone());
    }
    fn rotate_right(&mut self, this: Rc<RefCell<RBNode<K, V>>>) {
        if this.borrow().l_child.is_none() {
            return;
        }
        let p_node = this.borrow().parent.clone();
        // l_child safe to unwrap
        let l_child = this.borrow().l_child.as_ref().unwrap().clone();
        this.borrow_mut().color_swap(l_child.borrow_mut());

        // unsure if r_child is Some.. clone option
        this.borrow_mut().l_child = l_child.borrow().r_child.clone();
        l_child.borrow_mut().r_child = Some(this.clone());

        l_child.borrow_mut().parent = this.borrow().parent.clone();
        this.borrow_mut().parent = Some(l_child.clone());

        if p_node.is_none() {
            self.root = Some(l_child.clone());
            return;
        }
        let parent = p_node.unwrap();
        let mut parent = parent.borrow_mut();
        if let Some(parent_l_child) = &parent.l_child {
            if this.borrow().key == parent_l_child.borrow().key {
                parent.l_child = Some(l_child.clone());
                return;
            }
        }
        parent.r_child = Some(l_child.clone());
    }

    fn compare(node: Rc<RefCell<RBNode<K, V>>>, target: &K) -> i8 {
        if node.borrow().key > *target {
            return -1;
        } else if node.borrow().key < *target {
            return 1;
        } else {
            return 0;
        }
    }
    fn node_search(&self, key: &K) -> (i8, Option<Rc<RefCell<RBNode<K, V>>>>) {
        let mut this = self.root.clone();
        let mut candidate: Rc<RefCell<RBNode<K, V>>>;
        let mut result = 0;
        loop {
            match this {
                None => break,
                _ => {}
            }
            candidate = this.unwrap();
            result = RBTree::compare(candidate.clone(), key);
            let l_child = candidate.borrow().l_child.clone();
            let r_child = candidate.borrow().r_child.clone();
            if result != 0 {
                if result > 0 {
                    if r_child.is_none() {
                        return (result, Some(candidate));
                    }
                    this = r_child;
                } else {
                    if l_child.is_none() {
                        return (result, Some(candidate));
                    }
                    this = l_child;
                }
            } else {
                return (result, Some(candidate));
            }
        }
        (result, None)
    }

    pub fn get_node(&self, key: &K) -> Option<RBNode<K, V>> {
        let results = self.node_search(key);
        let result = results.0;
        if result == 0 {
            return Some(RBNode::from(results.1.as_ref().unwrap().clone()));
        }
        None
    }
    pub fn put(&mut self, key: K, data: V) {
        let mut node = RBNode::new(key, data);
        // node.data = data;
        self.put_node(node);
    }
    // pub fn traverse_and_print(&self) {
    //     let this = self.root.clone();
    //     if this.is_some() {
    //         self.print_tree(this.unwrap());
    //     } else {
    //         println!("empty");
    //     }
    // }
    // fn print_tree(&self, node: Rc<RefCell<RBNode>>) {
    //     if node.borrow().l_child.is_some() {
    //         let l_child = node.borrow().l_child.as_ref().unwrap().clone();
    //         self.print_tree(l_child);
    //         print!("left child: ");
    //     }
    //     if node.borrow().r_child.is_some() {
    //         let r_child = node.borrow().r_child.as_ref().unwrap().clone();
    //         self.print_tree(r_child);
    //         print!("right child: ");
    //     }
    //     println!("{} -> {}", node.borrow().key, node.borrow().data);
    // }
    pub fn put_node(&mut self, mut this: RBNode<K, V>) {
        // this.parent = None;
        // this.key = key;
        // this.color = Color::Red;
        // this.l_child = None;
        // this.r_child = None;

        if self.root.is_none() {
            let node = Rc::new(RefCell::new(this));
            node.borrow_mut().color = Color::Black;
            self.root = Some(node);
            return;
        }
        let results = self.node_search(&this.key);
        let node = results.1;
        let result = results.0;
        if result != 0 && node.is_some() {
            this.parent = Some(node.clone().unwrap());
            if result > 0 {
                node.unwrap().borrow_mut().r_child = Some(Rc::new(RefCell::new(this.clone())));
            } else {
                node.unwrap().borrow_mut().l_child = Some(Rc::new(RefCell::new(this.clone())));
            }
        } else if node.is_some() {
            // key already exists
            let node = node.unwrap();
            this.parent = node.borrow().parent.clone();
            node.borrow_mut().data = this.data;
            return;
        }
        while !this.is_root() {
            let parent = this.clone().parent.unwrap();
            if parent.borrow().color != Color::Red {
                break;
            }
            if this.uncle().is_some() {
                let uncle = this.uncle().unwrap();
                if uncle.borrow().color == Color::Red {
                    {
                        let grandparent = parent.borrow().parent.clone().unwrap();
                        let mut grandparent = grandparent.borrow_mut();
                        grandparent.color = Color::Red;
                        let mut parent = parent.borrow_mut();
                        let mut uncle = uncle.borrow_mut();
                        uncle.color = Color::Black;
                        parent.color = Color::Black;
                       
                    }
                    let grandparent = parent.borrow().parent.clone().unwrap();
                    this = RBNode::from(grandparent.clone());
                    continue;
                }
            }
            if this.clone().parent.unwrap().borrow().is_left_child() && this.is_left_child() {
                let parent = parent.borrow().clone();
                let grandparent = parent.parent.as_ref().unwrap();
                self.rotate_right(grandparent.clone());
            } else if this.clone().parent.unwrap().borrow().is_left_child() && this.is_right_child()
            {
                let parent = this.parent.as_ref().unwrap();
                self.rotate_left(parent.clone());
                self.rotate_right(parent.clone());
            } else if this.clone().parent.unwrap().borrow().is_right_child()
                && this.is_right_child()
            {
                let parent = parent.borrow().clone();
                let grandparent = parent.parent.as_ref().unwrap();
                self.rotate_left(grandparent.clone());
            } else {
                // this case causes issues due to
                // unhandled cases in rotation methods.
                // tree will not be balanced correctly.
                if parent.borrow().r_child.is_none() && parent.borrow().l_child.is_none() {
                    break;
                }
                self.rotate_right(parent.clone());
                self.rotate_left(parent);
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
    }
}
