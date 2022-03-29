use rbtree::RBTree

// Ideas for Sandbox module conversion

pub mod Sandbox {
    fn ALIGN(x: u32, a: u32) -> u32 {
        (((x) + ((a) - 1)) & !((a) - 1))
    }

    struct Sandbox {
        rb_tree RBTree, // TODO create rb node type
        size u32,
        buf_ptr u32,
        base u32,
        used u32,
        capacity u32
    }

    impl Sandbox {
        // sandbox_create
        fn new(u32 start, u32 size) -> Self {
            // TODO
        }

        // sandbox_reset
        fn reset(&mut self) {
            self.buf_ptr = self.base;
            self.used = 0;
            self.capacity = self.size;
        }

        // sandbox_bucket_allocate
        fn put_object(&mut self, size: u16, u32 align /* rb node */) -> u32 {
            let mut new_base = self.buf_ptr;
            let mut len: u32;

            if align == 2 {
                new_base = new_base | 0x2UL;
            } else {
                newBase = ALIGN(newBase, align)
            }
            len = newBase - self.buf_ptr + size;
            if self.capacity > len {
                self.capacity -= len;
                self.used += len;
                self.buf_ptr += len;
                // TODO insert node in self.rb_tree
            } else {
                new_base = 0;
            }

        }

        // sandbox_get_object
        fn get_object(&self, u32 address) /* Return Option<rb node> */ {

        }

        // there should be no need for sandbox_destroy 
    }
}
