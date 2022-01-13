extern crate cbindgen;

//use std::env;

mod errors;
mod utils;

use crate::utils::keys_utils::*;

fn main() {
    // let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let config = cbindgen::Config::from_file("cbindgen.toml").unwrap();

    // //println!("DEBUG----- {:?}", config);
    // cbindgen::Builder::new()
    //     .with_config(config)
    //     .with_crate(crate_dir)
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file("bindings/c/include/gix_guard_kit_rust.h");

    // let hex_key_arr = [
    //     0x0_u8, 0x37, 0x60, 0x8c, 0x14, 0xe6, 0xcd, 0xce, 0xc5, 0x09, 0x70, 0x45, 0xc6, 00, 0xf9,
    //     0x3f, 0x3d, 0xd4, 0x5d, 0xa9, 0x7b, 0x4c, 0x3a, 0xa5, 0x13, 0xde, 0x48, 0x8c, 0x95, 0xec,
    //     0xf6, 0x42,
    // ];
    let hex_key = "0037608c14e6cdcec5097045c600f93f3dd45da97b4c3aa513de488c95ecf642";
    // let ppp: Vec<u8> = (0..hex_key.len())
    //     .step_by(2)
    //     .map(|i| u8::from_str_radix(&hex_key[i..i + 2], 16).expect("---") ).collect()
    //     ;
    // let ppp: Vec<u8> = hex_key.chars().map(|i| u8::from_str_radix(format!("{}", i).as_str(), 16).expect("Great") ).collect();
    //     println!("ppp: {:?}", ppp);
    let ppp = hex_key.chars().map(|f| f as u8).collect();
    //let p = key_from_hex(Some( ppp ) );
    let p = key_from_hex(Some(ppp));
    match p {
        Ok(result) => {
            println!("Result: {:X?}", result);
        }
        Err(e) => print!("Failed. Error: {:?}", e),
    }
}
