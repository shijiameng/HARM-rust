use super::objects::Object;
use core::mem::MaybeUninit;

pub const NUM_OF_OBJECTS: usize = 1;

#[no_mangle]
pub static mut DISPATCH_TBL: MaybeUninit<[usize; NUM_OF_OBJECTS]> = MaybeUninit::uninit(); 

#[no_mangle]
pub static OBJECTS: [Object; NUM_OF_OBJECTS] = [
    // dummy object
    Object { 
        instance: None, 
        adjust_items: None,
        code: None,
        flags: 0,
        size: 0,
    },
];
