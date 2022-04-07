use super::adjustment::{AdjustItem, Branch};

pub const NUM_OF_BRANCHES: usize = 1;

#[no_mangle]
pub static BRANCHES: [AdjustItem::<Branch>; NUM_OF_BRANCHES] = [
    // dummy branch
    AdjustItem::<Branch> {
        offset: 0,
        item: Branch(0, 0),
    },
];