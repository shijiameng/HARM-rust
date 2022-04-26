use super::objects::{Object, ObjectKind};
use core::mem::MaybeUninit;

pub const NUM_OF_OBJECTS: usize = 0;


#[no_mangle]
pub static mut DISPATCH_TBL: MaybeUninit::<[u32; NUM_OF_OBJECTS]> = MaybeUninit::<[u32; NUM_OF_OBJECTS]>::uninit();

#[no_mangle]
pub static OBJECTS: [ObjectKind; NUM_OF_OBJECTS] = [];

pub const NUM_OF_VECTORS: usize = 0;

#[no_mangle]
pub static VECTORS: [&ObjectKind; NUM_OF_VECTORS] = [];
