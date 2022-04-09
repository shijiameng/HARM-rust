use super::rb_node::RBNode;
struct RBTree<'a> {
    root: Option<&'a RBNode<'a>>,
    size: u32
}