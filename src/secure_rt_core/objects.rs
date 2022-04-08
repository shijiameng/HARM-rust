use super::adjustment::{AdjustItem, Branch};
use super::codeblock::CodeBlock;
#[repr(C)]
pub struct Callsite {
    pub offset: u16,
    pub index: u16,
} 

#[repr (C)]
pub struct Object<'a> {
    pub instance: Option<&'a usize>,
    pub adjust_items: Option<&'a [AdjustItem::<Branch>]>,
    pub code: Option<&'a [u8]>,
    pub flags: u32,
    pub size: u16,
}

impl<'a> Object<'a> {
    #[inline]
    pub fn get_instance_address(&self) -> Option<usize> {
        match self.instance {
            Some(address) => Some(*address),
            _ => None,
        }
    }

    #[inline]
    pub fn get_address(&self) -> Option<usize> {
        if let Some(code) = self.code {
            Some(&code[0] as *const u8 as *const () as usize)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_instance(&self, mem: &'a mut [u8]) -> CodeBlock {
        let address = self.get_instance_address().unwrap();
        let base = &mem[0] as *const u8 as *const () as usize;
        let offset = address - base;

        CodeBlock::from(&mut mem[offset .. offset + self.size()])
    }

    #[inline]
    pub fn size(&self) -> usize {
        if let Some(code) = self.code {
            code.len()
        } else {
            0
        }
    }
}