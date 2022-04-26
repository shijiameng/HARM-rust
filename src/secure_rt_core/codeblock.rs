use core::mem::size_of_val;
use core::slice::from_raw_parts_mut;

#[repr(C)]
pub struct CodeBlock<'a> {
    pub block: &'a mut[u8],
}

impl<'a> CodeBlock<'a> {
    pub unsafe fn from(address: usize, length: usize) -> CodeBlock<'a> {
        CodeBlock {
            block: from_raw_parts_mut(address as *mut u8, length),
        }
    }

    #[inline]
    fn get_available_space(&self, offset: usize) -> usize {
        let length = size_of_val(self.block);
        
        if offset < length {
            length - offset
        } else {
            0
        }
    }

    pub fn size(&self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn get_address(&self, offset: usize) -> Result<usize, ()> {
        if offset < size_of_val(self.block) {
            Ok(&self.block[offset] as *const u8 as *const () as usize)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn read8(&self, offset: usize) -> Result<u8, ()> {
        if self.get_available_space(offset) >= 1 {
            Ok(self.block[offset])
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn read16(&self, offset: usize) -> Result<u16, ()> {
        if self.get_available_space(offset) >= 2 {
            Ok(unsafe { *(&self.block[offset] as *const u8 as *const u16) })
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn read32(&self, offset: usize) -> Result<u32, ()> {
        if self.get_available_space(offset) >= 4 {
            Ok(unsafe { *(&self.block[offset] as *const u8 as *const u32) })
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn write8(&mut self, offset: usize, value: u8) -> Result<u8, ()> {
        if self.get_available_space(offset) >= 1 {
            self.block[offset] = value;
            Ok(value)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn write16(&mut self, offset: usize, value: u16) -> Result<u16, ()> {
        if self.get_available_space(offset) >= 2 {
            unsafe { 
                *(&mut self.block[offset] as *mut u8 as *mut u16) = value;
            }
            Ok(value)
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn write32(&mut self, offset: usize, value: u32) -> Result<u32, ()> {
        if self.get_available_space(offset) >= 4 {
            unsafe { 
                *(&mut self.block[offset] as *mut u8 as *mut u32) = value;
            }
            Ok(value)
        } else {
            Err(())
        }
    }

    pub fn fill(&mut self, code: &[u8]) -> Result<(), ()> {
        if self.size() >= code.len() {
            self.block[..].copy_from_slice(&code[..]);
            Ok(())
        } else {
            Err(())
        }
    }
}