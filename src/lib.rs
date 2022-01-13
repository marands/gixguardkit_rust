pub mod errors;
pub mod utils;

pub use errors::GixTunnelErrorKind;
use std::os::raw::c_char;

use self::utils::keys_utils;
use self::utils::keys_utils::{GXT_KEY_LEN, GXT_KEY_LEN_HEX, GXT_KEY_LEN_BASE64};

#[no_mangle]
pub extern "C" fn key_from_base64(
    dst: *mut [u8; GXT_KEY_LEN],
    src: *const [c_char; GXT_KEY_LEN_BASE64],
) -> GixTunnelErrorKind {
    // remove C null terminated here from string ==> (GXT_KEY_LEN_BASE64 - 1)
    let base64_array: Vec<u8> =
        unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN_BASE64 - 1) }.to_vec();
    let base64_string = match String::from_utf8(base64_array) {
        Ok(x) => x,
        Err(e) => {
            print!("Failed to case input base64 to rust String. {:?}", e);
            return GixTunnelErrorKind::InvalidInput;
        }
    };
    let key: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN) };
    let res = keys_utils::key_from_base64(Some(base64_string));
    match res {
        Ok(out_vec) => {
            for (index, item) in out_vec.iter().enumerate() {
                key[index] = *item;
            }
            GixTunnelErrorKind::Ok
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn key_to_base64(
    dst: *mut [c_char; GXT_KEY_LEN_BASE64],
    src: *const [u8; GXT_KEY_LEN],
) -> GixTunnelErrorKind {
    let base64_array: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN_BASE64) };
    let key: &[u8] = unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN) };
    let res = keys_utils::key_to_base64(Some(key.to_vec().as_ref()));
    match res {
        Ok(out_vec) => {
            for (index, item) in out_vec.as_bytes().iter().enumerate() {
                base64_array[index] = *item;
            }
            base64_array[GXT_KEY_LEN_BASE64 - 1] = 0;
            GixTunnelErrorKind::Ok
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn hex_from_key(
    dst: *mut [u8; GXT_KEY_LEN],
    src: *const [c_char; GXT_KEY_LEN_HEX],
) -> GixTunnelErrorKind {
    let key: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN) };
    // C strings are null terminated, so we will remove last byte
    let hex_key_str: &[u8] = unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN_HEX - 1) };
    let res = keys_utils::key_from_hex(Some(hex_key_str.to_vec()));
    match res {
        Ok(out_vec) => {
            for (index, item) in out_vec.iter().enumerate() {
                key[index] = *item;
            }
            //            key[GXT_KEY_LEN - 1] = 0;
            GixTunnelErrorKind::Ok
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn key_to_hex(
    dst: *mut [c_char; GXT_KEY_LEN_HEX],
    src: &[u8; GXT_KEY_LEN],
) -> GixTunnelErrorKind {
    let hex: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN_HEX) };
    let res = keys_utils::key_to_hex(Some(&src.to_vec()));
    match res {
        Ok(out_vec) => {
            for (index, item) in out_vec.iter().enumerate() {
                hex[index] = *item;
            }
            hex[GXT_KEY_LEN_HEX - 1] = 0;
            GixTunnelErrorKind::Ok
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn key_eq(l_val: *const [u8; GXT_KEY_LEN], r_val: *const [u8; GXT_KEY_LEN]) -> bool {
    let lvar: &[u8] = unsafe { std::slice::from_raw_parts(l_val.cast(), GXT_KEY_LEN) };
    let rvar: &[u8] = unsafe { std::slice::from_raw_parts(r_val.cast(), GXT_KEY_LEN) };
    keys_utils::key_eq(Some(&lvar.to_vec()), Some(&rvar.to_vec()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_key_to_hex() {}
}
