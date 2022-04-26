//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::{File, read_dir};
use std::io::{Write, Error};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::ffi::OsString;
use cc::Build;

#[derive(Debug, Serialize, Deserialize)]
struct RelocInfo {
    src_offset: u16,
    dst_index: u16,
    dst_offset: u16,
}

impl RelocInfo {
    pub fn get_reloc_string(&self) -> String {
        format!("Branch(0x{:x}, {}, 0x{:x}),", self.src_offset, self.dst_index, self.dst_offset)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum ObjectKind {
    Function,
    VectorTable,
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectInfo {
    index: usize,
    name: String,
    kind: ObjectKind,
    reloc_items: Vec::<RelocInfo>,
    address: u32,
    size: u16, 
    isr: u8,   
}

impl ObjectInfo {
    pub fn get_object_string(&self, index: usize, adjtbl_from: usize) -> String {
        let kind_str = match self.kind {
            ObjectKind::Function => "ObjectKind::Function",
            ObjectKind::VectorTable => "ObjectKind::VectorTable",
        };

        let reloc_str = if !self.reloc_items.is_empty() {
            format!("Some(({}, {}))", adjtbl_from, adjtbl_from + self.reloc_items.len()) 
        } else {
            "None".to_string()
        };

        let obj_string = format!("{}(Object {{ reloc_items: {}, address: 0x{:x}, size: {}, index: {} }}),", 
                            kind_str, reloc_str, self.address, self.size, index);

        obj_string
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CallsiteInfo {
    caller: u16,
    offsets: Vec::<u16>,
}

fn generate_callsite_metadata() -> Result<(), Error> {
    let callsites_str = include_str!("metadata/callsites.yaml");
    let callsites: Vec::<CallsiteInfo> = serde_yaml::from_str(callsites_str).expect("failed to parse callsite metadata");
    let mut rettbl_file = File::create("src/secure_rt_core/ret_tbl.rs")?;
    let mut nsc_file = File::create("c_lib/nonsecure_entry/callsite_tbl.h")?;
    let mut n_callsites = 0usize;

    rettbl_file.write_all("use super::objects::Callsite;\n".as_bytes())?;
    rettbl_file.write_all("\n#[no_mangle]\n".as_bytes())?;
    rettbl_file.write_all("pub static CALLSITE_TBL: [Callsite; NUM_OF_CALLSITES] = [".as_bytes())?;
    for cs in callsites.iter() {
        for offset in cs.offsets.iter() {  
            rettbl_file.write_all(format!("\n\tCallsite {{ offset: 0x{:x}, caller: {} }},", offset, cs.caller).as_bytes())?;
            n_callsites += 1;
        }
    }
    rettbl_file.write_all("\n];\n\n".as_bytes())?;
    rettbl_file.write_all(format!("pub const NUM_OF_CALLSITES: usize = {};", n_callsites).as_bytes())?;

    nsc_file.write_all("#ifndef CALLSITE_TBL_H\n".as_bytes())?;
    nsc_file.write_all("#define CALLSITE_TBL_H\n".as_bytes())?;
    nsc_file.write_all(format!("\n#define CALLSITE_TABLE_SIZE {}\n\n", n_callsites).as_bytes())?;
    nsc_file.write_all("#endif /* CALLSITE_TBL_H */".as_bytes())?;

    Ok(())
}

fn generate_object_metadata() -> Result<(), Error> {
    let objects_str = include_str!("metadata/objects.yaml");

    // parse objects
    let objects: Vec::<ObjectInfo> = serde_yaml::from_str(objects_str).expect("failed to parse metadata");
    let n_objs = format!("pub const NUM_OF_OBJECTS: usize = {};\n\n", objects.len());
    let mut obj_file = File::create("src/secure_rt_core/obj_tbl.rs")?;
    let mut adj_file = File::create("src/secure_rt_core/adj_tbl.rs")?;
    let mut dptbl_file = File::create("c_lib/nonsecure_entry/dispatch_tbl.h")?;
    let mut reloc_offset = 0usize;
    let mut obj_index = 0usize;
    let mut vectors = Vec::<&ObjectInfo>::new();

    dptbl_file.write_all("#ifndef DISPATCH_TBL_H\n".as_bytes())?;
    dptbl_file.write_all("#define DISPATCH_TBL_H\n".as_bytes())?;
    dptbl_file.write_all(format!("\n#define DISPATCH_TABLE_SIZE {}\n", objects.len()).as_bytes())?;
    dptbl_file.write_all("#define DISPATCH_MAGIC 0x10000000\n".as_bytes())?;
    dptbl_file.write_all("#define DISPATCH_INDEX_BITS 12\n".as_bytes())?;
    dptbl_file.write_all("#endif /* DISPATCH_TBL_H */\n".as_bytes())?;

    obj_file.write_all("use super::objects::{Object, ObjectKind};\n".as_bytes())?;
    // obj_file.write_all("use super::adj_tbl::BRANCHES;\n".as_bytes())?;
    obj_file.write_all("use core::mem::MaybeUninit;\n".as_bytes())?;
    obj_file.write_all(n_objs.as_bytes())?;
    obj_file.write_all("\n#[no_mangle]\n".as_bytes())?;
    obj_file.write_all("pub static mut DISPATCH_TBL: MaybeUninit::<[u32; NUM_OF_OBJECTS]> = MaybeUninit::<[u32; NUM_OF_OBJECTS]>::uninit();\n".as_bytes())?;
    obj_file.write_all("\n#[no_mangle]\n".as_bytes())?;
    obj_file.write_all("pub static OBJECTS: [ObjectKind; NUM_OF_OBJECTS] = [".as_bytes())?;

    adj_file.write_all("use super::adjustment::Branch;\n\n".as_bytes())?;
    adj_file.write_all("#[no_mangle]\n".as_bytes())?;
    adj_file.write_all("pub static BRANCHES: [Branch; NUM_OF_BRANCHES] = [".as_bytes())?;

    for obj in objects.iter() {
        let objstr = obj.get_object_string(obj_index, reloc_offset);
        obj_file.write_all(format!("\n\t// {} - {}\n\t", obj_index, obj.name).as_bytes())?;
        obj_file.write_all(objstr.as_bytes())?;
        obj_index += 1;
        reloc_offset += obj.reloc_items.len();
        for adj in obj.reloc_items.iter() {
            let adjstr = adj.get_reloc_string();
            adj_file.write_all("\n\t".as_bytes())?;
            adj_file.write_all(adjstr.as_bytes())?;
        }

        if obj.isr != 0 {
            vectors.push(&obj);
        }
    }

    adj_file.write_all("\n];\n".as_bytes())?;

    let n_adjs = format!("\npub const NUM_OF_BRANCHES: usize = {};\n", reloc_offset);
    adj_file.write_all(n_adjs.as_bytes())?;

    vectors.sort_by(|a, b| a.isr.cmp(&b.isr));

    obj_file.write_all("\n];\n\n".as_bytes())?;
    obj_file.write_all(format!("pub const NUM_OF_VECTORS: usize = {};\n\n", vectors.len()).as_bytes())?;
    obj_file.write_all("#[no_mangle]\n".as_bytes())?;
    obj_file.write_all("pub static VECTORS: [&ObjectKind; NUM_OF_VECTORS] = [".as_bytes())?;

    for v in vectors.iter() {
        let i = v.index;
        obj_file.write_all(format!("\n\t&OBJECTS[{}],", i).as_bytes())?;
    }

    obj_file.write_all(format!("\n];\n").as_bytes())?;
    
    Ok(())
}


fn main() -> Result<(), Error>{
    generate_object_metadata()?;
    generate_callsite_metadata()?;

    let c_entry = read_dir("c_lib/nonsecure_entry").unwrap().filter_map(|f| {
        f.ok().and_then(|e| {
            let path = e.path();
            match path.extension() {
                Some(ext) if ext.eq(&OsString::from("c")) => Some(path),
                _ => None
            }
        })
    }).collect::<Vec<_>>();
    Build::new().files(&c_entry).flag("-mcmse").compile("libnsc.a");

    Build::new()
        .define("CPU_LPC55S69JBD100", None)
        .define("CPU_LPC55S69JBD100_cm33", None)
        .define("CPU_LPC55S69JBD100_cm33_core0", None)
        .define("NDEBUG", None)
        .include("c_lib/lpc55s69/cmsis")
        .include("c_lib/lpc55s69/board")
        .include("c_lib/lpc55s69/device")
        .include("c_lib/lpc55s69/drivers")
        .file("c_lib/lpc55s69/board/board.c")
        .file("c_lib/lpc55s69/board/clock_config.c")
        .file("c_lib/lpc55s69/board/peripherals.c")
        .file("c_lib/lpc55s69/board/pin_mux.c")
        .file("c_lib/lpc55s69/board/tzm_config.c")
        .file("c_lib/lpc55s69/device/system_LPC55S69_cm33_core0.c")
        .file("c_lib/lpc55s69/drivers/fsl_clock.c")
        .file("c_lib/lpc55s69/drivers/fsl_common.c")
        .file("c_lib/lpc55s69/drivers/fsl_flexcomm.c")
        .file("c_lib/lpc55s69/drivers/fsl_gpio.c")
        .file("c_lib/lpc55s69/drivers/fsl_i2c.c")
        .file("c_lib/lpc55s69/drivers/fsl_power.c")
        .file("c_lib/lpc55s69/drivers/fsl_reset.c")
        .file("c_lib/lpc55s69/drivers/fsl_rng.c")
        .file("c_lib/lpc55s69/drivers/fsl_spi.c")
        .file("c_lib/lpc55s69/drivers/fsl_usart.c")
        .flag("-fno-pic")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-mcmse")
        .archiver("arm-none-eabi-ar")
        .compile("libdevice.a");

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search=c_lib/lpc55s69/libs");
    // println!("cargo:rustc-link-search={}", out.join("c_lib/lpc55s69/libs").display());
    println!("cargo:rustc-link-arg=-lpower_hardabi");


    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=src/nonsecure_entry/*");
    println!("cargo:rustc-link-arg=-Wl,--cmse-implib");
    println!("cargo:rustc-link-arg=-Wl,--out-implib={}", out.join("libnsclib.o").display());

    Ok(())
}
