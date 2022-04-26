pub mod objects;
pub mod sandbox;
pub mod adjustment;
pub mod codeblock;
pub mod rb_tree;
// extern crate alloc;

use core::option::Option;
use cortex_m;

mod obj_tbl;
mod adj_tbl;
mod ret_tbl;

// pub mod adjustment;
// pub mod objects;
// pub mod sandbox;
// pub mod codeblock;

use sandbox::SandBox;
use objects::*;

static mut SHUFFLED_SEQUENCE: [u16; obj_tbl::NUM_OF_OBJECTS] = [0u16; obj_tbl::NUM_OF_OBJECTS];

fn get_shuffled_sequence<'a>() -> &'a [u16] {
    unsafe { &SHUFFLED_SEQUENCE[..] }
}

fn init() {
    for  i in 0 .. obj_tbl::NUM_OF_OBJECTS {
        unsafe { SHUFFLED_SEQUENCE[i] = i as u16 };
    }
}

fn update_dispatch_table(index: usize, new_addr: usize) {
    unsafe {
        let mut dispatch_tbl = obj_tbl::DISPATCH_TBL.assume_init();
        dispatch_tbl[index] = new_addr as u32;
    }
}

fn update_vtor_register(offset: u32) {
    unsafe {
        core::ptr::write_volatile(0xE002ED08 as *mut u32, offset);
    }
}

fn shuffle(sbox: &mut SandBox, retaddr: Option<usize>) -> Option<usize> {
    let seq = get_shuffled_sequence();
    let mut new_retaddr: Option<usize> = None;

    sbox.reset();

    for i in 0 .. seq.len() {
        let obj_i = seq[i] as usize;
        let object = &obj_tbl::OBJECTS[obj_i];
        let ret_offset = match object {
            ObjectKind::Function(obj) => {
                if retaddr.is_some() && new_retaddr.is_none() {
                    let obj_addr = obj.get_instance_address();
                    let ret_addr = retaddr.unwrap();
                    if ret_addr >= obj_addr && ret_addr < obj_addr + obj.get_size() {
                        Some(ret_addr - obj_addr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },

            _ => None,
        };

        let new_addr = sbox.push(object).unwrap();
        update_dispatch_table(obj_i, new_addr);

        if let Some(offset) = ret_offset {
            new_retaddr = Some(offset + new_addr);
        }
    }

    new_retaddr
}

fn do_adjust(object: &Object) {
    let reloc_items = object.get_reloc_items();
    if reloc_items.is_none() {
        return;
    }

    let mut cb = object.get_instance().unwrap();
    let adjust_items = reloc_items.unwrap();

    for i in 0 .. adjust_items.len() {
        let adjust_item = &adjust_items[i];
        let offset = adjust_item.0 as usize;
        let src_addr = cb.get_address(offset).unwrap();
        let src_code = cb.read32(offset).unwrap();
        // let branch = &adjust_item.item;
        let target_index = adjust_item.1 as usize;
        let target_offset = adjust_item.0 as usize;

        if let ObjectKind::Function(target_func) = &obj_tbl::OBJECTS[target_index] {
            let dst_addr = target_func.get_instance_address();
            let new_code = adjustment::adjust_direct_branch(src_code, src_addr, dst_addr + target_offset);
            cb.write32(offset, new_code).unwrap();
        }
    }
}

fn ref_adjust() {
    // update each entry of vector table
    match &obj_tbl::OBJECTS[0] {
        ObjectKind::VectorTable(ns_vector_tbl) => {
            let mut ns_vector_inst = ns_vector_tbl.get_instance().unwrap();
            
            for i in 0 .. obj_tbl::NUM_OF_VECTORS {
                if let ObjectKind::Function(isr) = &*obj_tbl::VECTORS[i] {
                    let ns_vector_addr = isr.get_instance_address();
                    ns_vector_inst.write32((i + 1) >> 2, (ns_vector_addr & 1usize) as u32).unwrap();
                }
            }

            update_vtor_register(ns_vector_tbl.get_instance_address() as u32);
        },

        _ => unreachable!(),
    }

    // update references in each function
    for i in 1 .. obj_tbl::NUM_OF_OBJECTS {
        let object = &obj_tbl::OBJECTS[i];
        match object {
            ObjectKind::Function(ns_func_obj) => do_adjust(ns_func_obj),
            _ => unreachable!(),
        }
    }
}


pub unsafe fn take_sandbox<'a>(address: usize, length: usize) -> SandBox<'a> {
    SandBox::take(address, length)
}

// pub fn get_object<'a>(index: usize) -> Option<&'a ObjectKind> {
//     if index < obj_tbl::NUM_OF_OBJECTS {
//         Some(&obj_tbl::OBJECTS[index])
//     } else {
//         None
//     }
// }


pub fn start(sandbox_addr: usize, length: usize) -> ! {

    init();

    let mut sandbox = unsafe { take_sandbox(sandbox_addr, length) };
    let ns_vector_obj = &obj_tbl::OBJECTS[0];

    shuffle(&mut sandbox, None);
    ref_adjust();

    if let ObjectKind::VectorTable(ns_vector_tbl) = ns_vector_obj {
        let ns_vector_inst = ns_vector_tbl.get_instance().unwrap();
        let msp = ns_vector_inst.read32(0).unwrap();

        unsafe {
            cortex_m::register::msp::write_ns(msp);
            let ns_reset_vector = (ns_vector_inst.read32(4).unwrap() & !1) as u32;
            cortex_m::asm::bx_ns(ns_reset_vector);
            unreachable!();
        }
    }

    unreachable!();
}