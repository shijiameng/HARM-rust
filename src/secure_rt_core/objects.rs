use super::adjustment::Branch;
use super::codeblock::CodeBlock;
use super::{obj_tbl, adj_tbl};

use core::ops::Deref;
use core::slice;


#[repr(C)]
pub struct Callsite {
    pub offset: u16,
    pub caller: u16,
} 


/// Object Description
#[repr (C)]
pub struct Object {
    /// Branch instructions that need to be adjusted
    pub reloc_items: Option<(u16, u16)>,
    /// Original address in the flash
    pub address: usize,
    /// Object size
    pub size: u16,
    /// Index of this object
    pub index: u16,
}

#[repr (C)]
pub enum ObjectKind {
    VectorTable(Object),
    Function(Object),
}

impl Deref for Object {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.address as *const u8, self.size as usize) }
    }
} 

impl Object {
    pub fn get_instance_address(&self) -> usize {
        // let dispatch_tbl = unsafe { obj_tbl::DISPATCH_TBL.assume_init() };
        // dispatch_tbl[self.index as usize] as usize
        unsafe {
            obj_tbl::DISPATCH_TBL[self.index as usize] as usize
        }
    }
    

    #[inline]
    pub fn get_address(&self) -> usize {
        self.address
    }

    pub fn get_instance(&self) -> Option<CodeBlock> {
        let address = self.get_instance_address();
        Some(unsafe { CodeBlock::from(address, self.size as usize) })
    }

    pub fn get_origin_code(&self) -> Option<CodeBlock> {
        Some(unsafe { CodeBlock::from(self.address, self.size as usize)})
    }

    pub fn get_size(&self) -> usize {
        self.size as usize
    }

    pub fn get_reloc_items(&self) -> Option<&[Branch]> {
        if let Some(item) = self.reloc_items {
            Some(&adj_tbl::BRANCHES[item.0 as usize.. item.1 as usize])
        } else {
            None
        }
    }
}