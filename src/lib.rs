pub mod errors;
pub mod utils;

// pub use errors::GixTunnelErrorKind;
// use std::os::raw::c_char;
//
// use self::utils::keys_utils;
// use self::utils::constants::{GXT_KEY_LEN, GXT_KEY_LEN_BASE64, GXT_KEY_LEN_HEX};
// use self::utils::x25519;

// #[no_mangle]
// pub extern "C" fn curve25519_shared_secret(
//     shared_secret: *mut [u8; GXT_KEY_LEN],
//     private_key: *const [u8; GXT_KEY_LEN],
//     public_key: *const [u8; GXT_KEY_LEN],
// ) {
//     let shared_secret_w: &mut [[u8; GXT_KEY_LEN]] = unsafe { std::slice::from_raw_parts_mut(shared_secret, GXT_KEY_LEN) };
//     let public_key_w = unsafe { std::slice::from_raw_parts(public_key, GXT_KEY_LEN) };
//     let private_key_w = unsafe { std::slice::from_raw_parts(private_key, GXT_KEY_LEN) };
//     x25519::curve25519_shared_secret(&mut shared_secret_w[0], private_key_w[0], public_key_w[0])
// }
//
// #[no_mangle]
// pub extern "C" fn curve25519_generate_private_key(
//     private_key: *mut [u8; GXT_KEY_LEN]
// ) {
//     let private_key_w: &mut [[u8; GXT_KEY_LEN]] = unsafe { std::slice::from_raw_parts_mut(private_key, GXT_KEY_LEN) };
//     x25519::curve25519_generate_private_key(&mut private_key_w[0]);
// }
//
// #[no_mangle]
// pub extern "C" fn curve25519_derive_public_key(
//     public_key: *mut [u8; GXT_KEY_LEN],
//     private_key: *const [u8; GXT_KEY_LEN],
// ) -> bool {
//     let public_key_w: &mut [[u8; GXT_KEY_LEN]] = unsafe { std::slice::from_raw_parts_mut(public_key, GXT_KEY_LEN) };
//     let private_key_w = unsafe { std::slice::from_raw_parts(private_key, GXT_KEY_LEN) };
//     x25519::curve25519_derive_public_key(&mut public_key_w[0], private_key_w[0])
// }
//
// #[no_mangle]
// pub extern "C" fn key_from_base64(
//     dst: *mut [u8; GXT_KEY_LEN],
//     src: *const [c_char; GXT_KEY_LEN_BASE64],
// ) -> GixTunnelErrorKind {
//     // remove C null terminated here from string ==> (GXT_KEY_LEN_BASE64 - 1)
//     let base64_array: Vec<u8> =
//         unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN_BASE64 - 1) }.to_vec();
//     let base64_string = match String::from_utf8(base64_array) {
//         Ok(x) => x,
//         Err(e) => {
//             print!("Failed to case input base64 to rust String. {:?}", e);
//             return GixTunnelErrorKind::InvalidInput;
//         }
//     };
//     let key: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN) };
//     let res = keys_utils::key_from_base64(Some(base64_string));
//     match res {
//         Ok(out_vec) => {
//             for (index, item) in out_vec.iter().enumerate() {
//                 key[index] = *item;
//             }
//             GixTunnelErrorKind::Ok
//         }
//         Err(e) => e,
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn key_to_base64(
//     dst: *mut [c_char; GXT_KEY_LEN_BASE64],
//     src: *const [u8; GXT_KEY_LEN],
// ) -> GixTunnelErrorKind {
//     let base64_array: &mut [u8] =
//         unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN_BASE64) };
//     let key: &[u8] = unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN) };
//     let res = keys_utils::key_to_base64(Some(key.to_vec().as_ref()));
//     match res {
//         Ok(out_vec) => {
//             for (index, item) in out_vec.as_bytes().iter().enumerate() {
//                 base64_array[index] = *item;
//             }
//             base64_array[GXT_KEY_LEN_BASE64 - 1] = 0;
//             GixTunnelErrorKind::Ok
//         }
//         Err(e) => e,
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn key_from_hex(
//     dst: *mut [u8; GXT_KEY_LEN],
//     src: *const [c_char; GXT_KEY_LEN_HEX],
// ) -> GixTunnelErrorKind {
//     let key: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN) };
//     // C strings are null terminated, so we will remove last byte
//     let hex_key_str: &[u8] = unsafe { std::slice::from_raw_parts(src.cast(), GXT_KEY_LEN_HEX - 1) };
//     let res = keys_utils::key_from_hex(Some(hex_key_str.to_vec()));
//     match res {
//         Ok(out_vec) => {
//             for (index, item) in out_vec.iter().enumerate() {
//                 key[index] = *item;
//             }
//             //            key[GXT_KEY_LEN - 1] = 0;
//             GixTunnelErrorKind::Ok
//         }
//         Err(e) => e,
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn key_to_hex(
//     dst: *mut [c_char; GXT_KEY_LEN_HEX],
//     src: &[u8; GXT_KEY_LEN],
// ) -> GixTunnelErrorKind {
//     let hex: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dst.cast(), GXT_KEY_LEN_HEX) };
//     let res = keys_utils::key_to_hex(Some(&src.to_vec()));
//     match res {
//         Ok(out_vec) => {
//             for (index, item) in out_vec.iter().enumerate() {
//                 hex[index] = *item;
//             }
//             hex[GXT_KEY_LEN_HEX - 1] = 0;
//             GixTunnelErrorKind::Ok
//         }
//         Err(e) => e,
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn key_eq(l_val: *const [u8; GXT_KEY_LEN], r_val: *const [u8; GXT_KEY_LEN]) -> bool {
//     let lvar: &[u8] = unsafe { std::slice::from_raw_parts(l_val.cast(), GXT_KEY_LEN) };
//     let rvar: &[u8] = unsafe { std::slice::from_raw_parts(r_val.cast(), GXT_KEY_LEN) };
//     keys_utils::key_eq(Some(&lvar.to_vec()), Some(&rvar.to_vec()))
// }

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_key_to_hex() {}
}

include!(concat!(env!("OUT_DIR"), "/gix_guard.uniffi.rs"));
