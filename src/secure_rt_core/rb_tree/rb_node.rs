extern crate alloc;

use core::cell::RefCell;
use core::cell::RefMut;
use alloc::rc::Rc;
use core::marker::Copy;

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Clone)]
pub struct RBNode<K, V> {
    pub parent: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub l_child: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub r_child: Option<Rc<RefCell<RBNode<K, V>>>>,
    pub data: V, // CHANGE ME
    pub key: K,
    pub color: Color,
}

impl<K, V> RBNode<K, V> 
        where K: Clone + PartialEq + PartialOrd + core::marker::Copy, V: Clone {
    pub fn new(key: K, data: V) -> Self {
        Self {
            parent: None,
            r_child: None,
            l_child: None,
            color: Color::Black,
            data,
            key,
        }
    }
    pub fn from(this: Rc<RefCell<Self>>) -> Self {
        // where K_: Clone + PartialEq + PartialOrd + Copy, V_: Clone {
        let this = this.borrow();
        Self {
            parent: this.parent.clone(),
            l_child: this.l_child.clone(),
            r_child: this.r_child.clone(),
            data: this.data.clone(),
            key: this.key,
            color: this.color,
        }
    }
    pub fn is_root(&self) -> bool {
        match self.parent {
            Some(_) => false,
            None => true,
        }
    }

    pub fn is_left_child(&self) -> bool {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            if let Some(l_child) = &parent.l_child {
                return self.key == l_child.borrow().key;
            }
        }
        false
    }

    pub fn is_right_child(&self) -> bool {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            if let Some(r_child) = &parent.r_child {
                return self.key == r_child.borrow().key;
            }
        }
        false
    }

    pub fn grandparent(&self) -> Option<Rc<RefCell<Self>>> {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            return parent.parent.clone();
        }
        None
    }

    pub fn uncle(&self) -> Option<Rc<RefCell<Self>>> {
        match self.grandparent() {
            Some(grandparent) => {
                // self has grandparent, parent is safe to unwrap
                if self.parent.clone().unwrap().borrow().is_left_child() {
                    grandparent.borrow().r_child.clone()
                } else {
                    grandparent.borrow().l_child.clone()
                }
            }
            None => None,
        }
    }

    pub fn color_swap(&mut self, mut other: RefMut<Self>) {
        let temp = self.color;
        self.color = other.color;
        other.color = temp;
    }
}
