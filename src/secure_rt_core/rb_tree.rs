enum Color {
    Red,
    Black,
}
struct RBNode {
    parent: Option<&RBNode>,
    l_child: Option<&RBNode>,
    r_child: Option<&RBNode>,
    data: std::ffi::c_void,
    color: Color
}

struct RBTree {
    root: Option<&RBNode>,
    size: u32
}

impl RBNode {
    fn is_root(&self) -> bool {
        match self.parent {
            Some => false,
            None => true,
        }
    }

    fn is_left_child(&self) -> bool {
        match self.parent {
            Some(parent) => {
                match parent.l_child {
                    Some(l_child) => self == l_child
                    None => false,
                }
            }
            None => false,
        }
    }

    fn is_right_child(&self) -> bool {
        match self.parent {
            Some(parent) => {
                match parent.r_child {
                    Some(r_child) => self == r_child
                    None => false,
                }
            }
            None => false,
        }
    }

    fn grandparent(&self) => Option<&RBNode> {
        match self.parent {
            Some(parent) => {
                parent.parent
            }
            None => None
        }
    }

    fn uncle(&self) => Option<&RBNode> {
        match self.grandparent() {
            Some(grandparent) => {
                if self.parent.unwrap().is_left_child() {
                    grandparent.r_child
                } 
                grandparent.l_child
            },
            None => None
        }
    }

    fn color_swap(&mut self, &mut Self) {
        let temp = self.color;
        self.color = Self.color;
        Self.color = temp;
    }
}

