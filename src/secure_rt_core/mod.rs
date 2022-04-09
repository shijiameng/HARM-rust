pub mod objects;
pub mod sandbox;
pub mod adjustment;
pub mod codeblock;

// extern crate alloc;

use core::mem::MaybeUninit;
use core::option::Option;
use cortex_m;

mod obj_tbl;
mod adj_tbl;
mod ret_tbl;

use sandbox::Sandbox;
use objects::*;

#[link_section = ".nonsecure.code"]
static NONSECURE_FLASH: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();

#[link_section = ".nonsecure.sandbox"]
static mut NONSECURE_SANDBOX: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();

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
        dispatch_tbl[index] = new_addr;
    }
}

fn update_vtor_register(offset: u32) {
    unsafe {
        core::ptr::write_volatile(0xE002ED08 as *mut u32, offset);
    }
}

fn do_shuffle(sbox: &mut Sandbox, object: &Object, is_vector: bool) -> usize {
    // Get code block from the flash memory
    let orign_addr = object.get_address().unwrap();
    // Allocate a code block from the sandbox
    let align_bits = if is_vector { 7 } else if orign_addr & !3 == orign_addr { 2 } else { 1 };
    let mut new_cb = sbox.get_block(object.size as usize, align_bits).unwrap();
    
    new_cb.fill(&object.code.unwrap()).unwrap();
    new_cb.get_address(0).unwrap()
}

fn shuffle(sbox: &mut Sandbox, brk_point: Option<usize>) -> Option<usize> {
    let seq = get_shuffled_sequence();
    let mut ret_addr: Option<usize> = None;

    sbox.reset();

    for i in 0 .. seq.len() {
        let obj_i = seq[i] as usize;
        let object = get_object(obj_i).unwrap();
        let brk_offset = {
            if brk_point.is_some() && ret_addr.is_none() {
                let brk_addr = brk_point.unwrap();
                let obj_addr = object.get_instance_address().unwrap();
                if brk_addr >= obj_addr && brk_addr < obj_addr + object.size() {
                    Some(brk_addr - obj_addr)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let new_addr = do_shuffle(sbox, &object, obj_i == 0);

        update_dispatch_table(obj_i, new_addr);

        if let Some(offset) = brk_offset {
            ret_addr = Some(new_addr + offset);
        }
    }

    ret_addr
}

fn do_adjust(object: &Object) {
    if object.adjust_items.is_none() {
        return;
    }

    let adjust_items = object.adjust_items.unwrap();
    let mut nonsecure_sandbox = unsafe { NONSECURE_FLASH.assume_init() };
    let mut cb = object.get_instance(&mut nonsecure_sandbox[..]);
    
    for i in 0 .. adjust_items.len() {
        let adjust_item = &adjust_items[i];
        let offset = adjust_item.offset as usize;
        let src_addr = cb.get_address(offset).unwrap();
        let src_code = cb.read32(offset).unwrap();
        let branch = &adjust_item.item;
        let target_index = branch.1 as usize;
        let target_offset = branch.0 as usize;
        let target_object = get_object(target_index).unwrap();
        let dst_addr = target_object.get_instance_address().unwrap();
        let new_code = adjustment::adjust_direct_branch(src_code, src_addr, dst_addr + target_offset);
        cb.write32(offset, new_code).unwrap();
    }
}

fn ref_adjust() {
    let vector_obj = &obj_tbl::OBJECTS[0];
    update_vtor_register(vector_obj.get_instance_address().unwrap() as u32);

    for i in 1 .. obj_tbl::NUM_OF_OBJECTS {
        let object = &obj_tbl::OBJECTS[i];
        do_adjust(object);
    }
}


pub fn take_sandbox<'a>(memory: &'a mut [u8]) -> Sandbox<'a> {
    Sandbox::new(memory)
}

pub fn get_object<'a>(index: usize) -> Option<&'a Object<'a>> {
    if index < obj_tbl::NUM_OF_OBJECTS {
        Some(&obj_tbl::OBJECTS[index])
    } else {
        None
    }
}


pub fn start() {

    init();

    let mut sandbox_mem = unsafe { NONSECURE_SANDBOX.assume_init() };
    let mut sandbox = take_sandbox(&mut sandbox_mem);
    let ns_vector_obj = &obj_tbl::OBJECTS[0];

    shuffle(&mut sandbox, None).unwrap();
    ref_adjust();

    let ns_vector_cb = ns_vector_obj.get_instance(&mut sandbox_mem);
    let msp = ns_vector_cb.read32(0).unwrap();

    unsafe {
        cortex_m::register::msp::write_ns(msp);
        let ns_reset_vector = (ns_vector_cb.get_address(4).unwrap() & !1) as u32;
        cortex_m::asm::bx_ns(ns_reset_vector);
        unreachable!()
    }
}