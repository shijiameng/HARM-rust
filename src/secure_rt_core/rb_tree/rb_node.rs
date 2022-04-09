#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Color {
    Red,
    Black,
}

pub struct RBNode<'a> {
    parent: Option<&'a RBNode<'a>>,
    l_child: Option<&'a RBNode<'a>>,
    r_child: Option<&'a RBNode<'a>>,
    data: u32,
    color: Color
}
impl PartialEq for RBNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.data == other.data
    }
}
impl <'a> RBNode<'a> {
    pub fn is_root(&self) -> bool {
        match self.parent {
            Some(_) => false,
            None => true,
        }
    }

    pub fn is_left_child(&self) -> bool {
        match self.parent {
            Some(parent) => {
                match parent.l_child {
                    Some(l_child) => self == l_child,
                    None => false,
                }
            }
            None => false,
        }
    }

    pub fn is_right_child(&self) -> bool {
        match self.parent {
            Some(parent) => {
                match parent.r_child {
                    Some(r_child) => self == r_child,
                    None => false,
                }
            }
            None => false,
        }
    }

    pub fn grandparent(&self) -> Option<&RBNode> {
        match self.parent {
            Some(parent) => {
                parent.parent
            }
            None => None
        }
    }

    pub fn uncle(&self) -> Option<&RBNode> {
        match self.grandparent() {
            Some(grandparent) => {
                if self.parent.unwrap().is_left_child() {
                    grandparent.r_child
                } 
                else {
                    grandparent.l_child
                }
            },
            None => None
        }
    }

    pub fn color_swap(&mut self, other: &mut Self) {
        let temp = self.color;
        self.color = other.color;
        other.color = temp;
    }
}

