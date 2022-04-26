pub struct Branch(pub u16, pub u16, pub u16);

fn encode_B_T4(src_addr: u32, dst_addr: u32) -> u32 {
    let offset: i32 = dst_addr as i32 - src_addr as i32 - 4;
    let s: u32 = if offset < 0 { 1 } else { 0 };
    let i1: u32 = if offset & (1 << 23) > 0 { 1 } else { 0 };
    let i2: u32 = if offset & (1 << 22) > 0 { 1 } else { 0 };
    let imm10: u32 = ((offset >> 12) & 0b1111111111) as u32;
    let imm11: u32 = ((offset >> 1) & 0b11111111111) as u32;
    let j1: u32 = ((!i1 & 1u32) ^ s) & 1u32;
    let j2: u32 = ((!i2 & 1u32) ^ s) & 1u32;

    (0b11110 << 27)
            | (s << 26)
            | (imm10 << 16)
            | (1 << 15)
            | (j1 << 13)
            | (1 << 12)
            | (j2 << 11)
            | imm11
}

fn encode_B_T3(src_addr: u32, dst_addr: u32, cc: u8) -> u32 {
    let offset: i32 = dst_addr as i32 - src_addr as i32 - 4;
    let s: u32 = if offset < 0 { 1 } else { 0 };
    let j1: u32 = if offset & (1 << 18) != 0 { 1 } else { 0 }; 
    let j2: u32 = if offset & (1 << 19) != 0 { 1 } else { 0 };
    let imm6: u32 = ((offset >> 12) & 0b111111) as u32;
    let imm11: u32 = ((offset >> 1) & 0b11111111111) as u32;
    
    (0b11110 << 27)
            | (s << 26)
            | ((cc as u32 & 0b1111) << 22)
            | (imm6 << 16)
            | (1 << 15)
            | (j1 << 13)
            | (j2 << 11)
            | imm11
}

pub fn adjust_direct_branch(src_code: u32, src_addr: usize, dst_addr: usize) -> u32 {
    if src_code & (1 << 28) == 0 {
        let cc = (src_code >> 6) & 0b1111;
        encode_B_T3(src_addr as u32, dst_addr as u32, cc as u8)
    } else {
        encode_B_T4(src_addr as u32, dst_addr as u32)
    }
}