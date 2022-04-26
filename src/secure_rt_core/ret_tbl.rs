use super::objects::Callsite;

#[no_mangle]
pub static CALLSITE_TBL: [Callsite; NUM_OF_CALLSITES] = [];

pub const NUM_OF_CALLSITES: usize = 0;