use super::rb_node::*;
use std::cell::RefCell;
use std::rc::Rc;

struct RBTree {
    root: Option<Rc<RefCell<RBNode>>>,
    size: u32,
}

impl RBTree {
    fn rotate_left(&mut self, this: Rc<RefCell<RBNode>>) {
        let r_child = &this.borrow().r_child;
        let parent = &this.borrow().parent;
        if r_child.is_none() {
            return;
        }
        let r_child = r_child.as_ref().unwrap();
        // r_child is safe to unwrap
        this.borrow_mut().color_swap(r_child.borrow_mut());
        // unsure if l_child is Some... clone the option
        this.borrow_mut().r_child = r_child.borrow().l_child.clone();
        r_child.borrow_mut().l_child = Some(this.clone());

        // unsure if parent is Some... clone the option
        r_child.borrow_mut().parent = this.borrow().parent.clone();
        this.borrow_mut().parent = Some(r_child.clone());
        if parent.is_none() {
            self.root = Some(r_child.clone());
            return;
        }
        // parent is safe to unwrap
        let mut parent = parent.as_ref().unwrap().borrow_mut();
        // is parent l_child safe to unwrap?
        if let Some(parent_l_child) = &parent.l_child {
            if this.borrow().key == parent_l_child.borrow().key {
                parent.l_child = Some(r_child.clone());
                return;
            }
        }
        parent.r_child = Some(r_child.clone());
    }
    fn rotate_right(&mut self, this: Rc<RefCell<RBNode>>) {
        let l_child = &this.borrow().l_child;
        let parent = &this.borrow().parent;

        if l_child.is_none() {
            return;
        }
        // l_child safe to unwrap
        let l_child = l_child.as_ref().unwrap();
        this.borrow_mut().color_swap(l_child.borrow_mut());

        // unsure if r_child is Some.. clone option
        this.borrow_mut().l_child = l_child.borrow().r_child.clone();
        l_child.borrow_mut().r_child = Some(this.clone());

        l_child.borrow_mut().parent = this.borrow().parent.clone();
        this.borrow_mut().parent = Some(l_child.clone());

        if parent.is_none() {
            self.root = Some(l_child.clone());
            return;
        }
        let mut parent = parent.as_ref().unwrap().borrow_mut();
        if let Some(parent_l_child) = &parent.l_child {
            if this.borrow().key == parent_l_child.borrow().key {
                parent.l_child = Some(l_child.clone());
                return;
            }
        }
        parent.r_child = Some(l_child.clone());
    }

    fn compare(node: Rc<RefCell<RBNode>>, key: u32) -> usize {
        // TODO
        0
    }
    fn node_search(&self, key: u32) -> (usize, Option<Rc<RefCell<RBNode>>>) {
        let mut this = self.root.clone();
        let mut candidate: Rc<RefCell<RBNode>>;
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
                    this = r_child;
                } else {
                    this = l_child;
                }
            } else {
                return (result, Some(candidate));
            }
        }
        (result, this)
    }

    pub fn put(&mut self, key: u32, mut this: RBNode) {
        this.parent = None;
        this.color = Color::Red;
        this.l_child = None;
        this.r_child = None;

        if self.root.is_none() {
            let node = Rc::new(RefCell::new(this));
            node.borrow_mut().color = Color::Black;
            self.root = Some(node);
            return;
        }
        let results = self.node_search(key);
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
            let mut node = node.borrow_mut();
            if node.is_left_child() {
                this.parent.unwrap().borrow_mut().l_child =
                    Some(Rc::new(RefCell::new(this.clone())));
            } else {
                this.parent.unwrap().borrow_mut().r_child =
                    Some(Rc::new(RefCell::new(this.clone())));
            }
            node.parent = None;
            node.l_child = None;
            node.r_child = None;
            return;
        }
        while !this.is_root() {
            if let Some(parent) = this.clone().parent {
                if parent.borrow().color != Color::Red {
                    break;
                }
            } else {
                break;
            }
            if this.uncle().is_none() {
                break;
            }
            // parent, uncle, and grandparent safe to unwrap
            let mut uncle = this.uncle().unwrap().borrow_mut();
            let mut parent = this.clone().parent.unwrap().borrow_mut();
            let mut grandparent = parent.parent.clone().unwrap().borrow_mut();
            if uncle.color == Color::Red {
                uncle.color = Color::Black;
                parent.color = Color::Black;
                grandparent.color = Color::Red;
                let grandparent = parent.parent.as_ref().unwrap();
                this = RBNode::from(grandparent.clone());
                continue
            }
            if parent.is_left_child() && this.is_left_child() {
                let grandparent = parent.parent.as_ref().unwrap();
                self.rotate_right(grandparent.clone());
            } else if parent.is_left_child() && this.is_right_child() {
                let parent = this.parent.as_ref().unwrap();
                self.rotate_left(parent.clone());
                self.rotate_right(parent.clone());
            } else if parent.is_right_child() && this.is_right_child() {
                let grandparent = parent.parent.as_ref().unwrap();
                self.rotate_left(grandparent.clone());
            } else {
                let parent = this.parent.as_ref().unwrap();
                self.rotate_right(parent.clone());
                self.rotate_left(parent.clone());
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
    }
}
