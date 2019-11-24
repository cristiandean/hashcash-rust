extern crate sha1;

use sha1::Sha1;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

fn is_valid_hashcash(last_proof: &str, proof: u64, zeros: u32) -> bool {
    let mut hasher = Sha1::new();
    let guess = last_proof.to_owned() + proof.to_string().as_ref();
    hasher.update(guess.as_bytes());
    let result = hasher.digest().to_string();
    &result[..zeros as usize] == "0".repeat(zeros as usize)
}

#[no_mangle]
pub extern "C" fn digest(data: *mut c_char, zeros: u32) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(data);
        let mut proof: u64 = 0;
        let str_data = &data.to_string_lossy().into_owned();
        while !is_valid_hashcash(str_data, proof, zeros) {
            proof += 1;
        }
        let s = CString::new(proof.to_string()).unwrap();
        s.into_raw()
    }
}
