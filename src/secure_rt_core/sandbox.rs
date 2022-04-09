use super::objects::Object;
use super::codeblock::CodeBlock;
use rbtree::RBTree;
use std::cmp::Ordering;

pub struct Sandbox<'a> {
    rb_tree: RBTree<u32, &'a Object<'a>>,
    memory: &'a mut[u8],
    next_i: usize,
    capacity: usize,
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

impl <'a> Sandbox<'a> {
    // sandbox_create
    pub fn new(memory: &'a mut [u8]) -> Self {
        let sandbox_size = memory.len();
        Sandbox {
            rb_tree: RBTree::new(),
            memory,
            next_i: 0,
            capacity: sandbox_size,
        }
    }

    fn align(&self, x: u32, a: u32) -> u32 {
        ((x) + ((a) - 1)) & !((a) - 1)
    }
    // sandbox_reset
    pub fn reset(&mut self) {
        let sbox_len = self.memory.len();
        self.next_i = 0;
        self.capacity = sbox_len;
    }

    pub fn get_block(&mut self, block_size: usize, align_bits: u8) -> Result<CodeBlock, ()> {
        let align_bytes: usize = 1 << align_bits;
        let block_base: usize = (self.next_i + (align_bytes - 1)) & !(align_bytes - 1);
        let actual_size = block_base - self.next_i + block_size;

        if self.capacity >= actual_size {
            self.capacity -= actual_size;
            self.next_i += actual_size;
            Ok(CodeBlock::from(&mut self.memory[block_base .. block_base + block_size]))
        } else {
            Err(())
        }
    }

    // sandbox_bucket_allocate
    pub fn put_object(&mut self, node: &'a Object<'a>, align_bits: u8) -> Result<CodeBlock, ()> {
        let object_address = node.get_address().unwrap();
        let mut block = self.get_block(node.size(), align_bits).unwrap();

        block.fill(node.code.unwrap());
        self.rb_tree.insert(block.get_address(0).unwrap() as u32, node);

        Ok(block)
    }

    // sandbox_get_object
    // pub fn get_object(&self, address: u32) -> Option<&Object<'a>> {
    //     // self.rb_tree.get(&address)
    // }

}
