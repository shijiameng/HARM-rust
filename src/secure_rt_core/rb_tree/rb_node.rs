use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Clone)]
pub struct RBNode {
    pub parent: Option<Rc<RefCell<RBNode>>>,
    pub l_child: Option<Rc<RefCell<RBNode>>>,
    pub r_child: Option<Rc<RefCell<RBNode>>>,
    pub data: u32,
    pub key: u32,
    pub color: Color,
}

impl RBNode {
    pub fn from(this: Rc<RefCell<Self>>) -> Self {
        let this = this.borrow();
        Self {
            parent: this.parent.clone(),
            l_child: this.l_child.clone(),
            r_child: this.r_child.clone(),
            data: this.data,
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
                let l_child = l_child.borrow();
                return self.key == l_child.key;
            }
        }
        false
    }

    pub fn is_right_child(&self) -> bool {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            if let Some(r_child) = &parent.r_child {
                let r_child = r_child.borrow();
                return self.key == r_child.key;
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
