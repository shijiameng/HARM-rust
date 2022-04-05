// Ideas for Sandbox module conversion

use rbtree::RBTree;

pub struct Sandbox {
    rb_tree: Option<RBTree<u32, u32>>,
    size: u32,
    buf_ptr: u32,
    base: u32,
    used: u32,
    capacity: u32,
}

impl Sandbox {
    // sandbox_create
    pub fn new(start: u32, size: u32) -> Self {
        Sandbox {
            rb_tree: None,
            size,
            base: start,
            used: 0,
            capacity: size,
            buf_ptr: start,
        }
    }

    fn align(&self, x: u32, a: u32) -> u32 {
        ((x) + ((a) - 1)) & !((a) - 1)
    }
    pub fn say_hello() {
        println!("hello from sandbox");
    }
    // sandbox_reset
    pub fn reset(&mut self) {
        self.buf_ptr = self.base;
        self.used = 0;
        self.capacity = self.size;
    }

    // sandbox_bucket_allocate
    pub fn put_object(&mut self, size: u16, align: u32 /* rb node */) -> u32 {
        let mut new_base = self.buf_ptr;
        let mut len: u32;

        if align == 2 {
            new_base = new_base | 0x2u32;
        } else {
            new_base = self.align(new_base, align);
        }
        len = new_base - self.buf_ptr + (size as u32);
        if self.capacity > len {
            self.capacity -= len;
            self.used += len;
            self.buf_ptr += len;
            // TODO insert node in self.rb_tree
        } else {
            new_base = 0;
        }
        new_base
    }

    // sandbox_get_object
    pub fn get_object(&self, address: u32) /* Return Option<rb node> */ {}

    // there should be no need for sandbox_destroy
}
