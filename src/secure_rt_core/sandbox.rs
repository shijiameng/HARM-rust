use super::objects::{Object, ObjectKind};
use core::slice::from_raw_parts_mut;
use core::cmp::Ordering;

use super::rb_tree::rb_tree::RBTree;

/// Sandbox Struct
pub struct SandBox<'a> {
    /// the memory base of sandbox
    memory: &'a mut[u8],

    /// pointer of next availiable address
    next_ptr: usize,

    /// capacity of the sandbox
    capacity: usize,

    /// An Red-Black Tree that used to index all functions
    index: RBTree<usize, &'a ObjectKind>,
}


#[derive(Eq)]
struct Key {
    key: u32,
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}


impl<'a> SandBox<'a> {
    pub unsafe fn take(base: usize, size: usize) -> Self {
        SandBox {
            memory: from_raw_parts_mut(base as *mut u8, size),
            next_ptr: base,
            capacity: size,
            index: RBTree::<usize, &'a ObjectKind>::new(),
        }
    }

    fn get_base(&self) -> usize {
        self.memory.as_ptr() as usize
    }

    fn get_block(&mut self, block_size: usize, align_bits: u8) -> Result<&mut [u8], ()> {
        let align_bytes: usize = 1 << align_bits;
        let block_base: usize = (self.next_ptr + (align_bytes - 1)) & !(align_bytes - 1);
        let actual_size = block_base - self.next_ptr + block_size;
        let offset_i = block_base - self.get_base();

        // allocate a block from the sandbox
        if self.capacity >= actual_size {
            self.capacity -= actual_size;
            self.next_ptr += actual_size;
            Ok(&mut self.memory[offset_i .. offset_i + block_size])
        } else {
            Err(())
        }
    }

    pub fn push(&mut self, object: &'static ObjectKind) -> Result<usize, ()> {
        let obj: (&Object, u8) = match object {
            ObjectKind::VectorTable(obj) => (obj, 7),
            ObjectKind::Function(obj) => {
                let address = obj.get_address();
                if address & !3 == address { (obj, 2) } else { (obj, 1) }
            },
        };

        // copy the object code to the sandbox

        if let Ok(block) = self.get_block(obj.0.get_size() as usize, obj.1) {
            block.copy_from_slice(&**obj.0);
            let address = block as *const _ as *const u8 as usize;
            // self.index.put(address, object);
            Ok(address)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        let sbox_len = self.memory.len();
        self.next_ptr = self.get_base();
        self.capacity = sbox_len;
    }
}

// impl <'a> SandBox<'a> {
//     // sandbox_create
//     pub fn new(memory: &'a mut [u8]) -> Self {
//         let sandbox_size = memory.len();
//         SandBox {
//             rb_tree: RBTree::new(),
//             memory,
//             next_i: 0,
//             capacity: sandbox_size,
//         }
//     }

//     fn align(&self, x: u32, a: u32) -> u32 {
//         ((x) + ((a) - 1)) & !((a) - 1)
//     }
//     // sandbox_reset
//     pub fn reset(&mut self) {
//         let sbox_len = self.memory.len();
//         self.next_i = 0;
//         self.capacity = sbox_len;
//     }

//     pub fn get_block(&mut self, block_size: usize, align_bits: u8) -> Result<CodeBlock, ()> {
//         let align_bytes: usize = 1 << align_bits;
//         let block_base: usize = (self.next_i + (align_bytes - 1)) & !(align_bytes - 1);
//         let actual_size = block_base - self.next_i + block_size;

//         if self.capacity >= actual_size {
//             self.capacity -= actual_size;
//             self.next_i += actual_size;
//             Ok(CodeBlock::from(&mut self.memory[block_base .. block_base + block_size]))
//         } else {
//             Err(())
//         }
//     }

//     // sandbox_bucket_allocate
//     pub fn put_object(&mut self, node: &'a Object<'a>, align_bits: u8) -> Result<CodeBlock, ()> {
//         let object_address = node.get_address().unwrap();
//         let mut block = self.get_block(node.size(), align_bits).unwrap();

//         block.fill(node.code.unwrap());
//         self.rb_tree.insert(block.get_address(0).unwrap() as u32, node);

//         Ok(block)
//     }

//     // sandbox_get_object
//     // pub fn get_object(&self, address: u32) -> Option<&Object<'a>> {
//     //     // self.rb_tree.get(&address)
//     // }

// }
