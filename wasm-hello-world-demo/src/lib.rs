use std::ffi::CString;

#[no_mangle]
pub extern "C" fn wasm_hello(_input: *const u8, _len: usize) -> *const u32 {
    let cstring = match CString::new("200hello world!".to_owned()) {
        Ok(cstr) => cstr,
        Err(_) => return std::ptr::null_mut(),
    };

    let s_len = cstring.to_bytes().len() as u32;
    let s_ptr = cstring.into_raw() as u32;

    let data: [u32; 2] = [s_ptr as u32, s_len as u32];

    Box::into_raw(Box::new(data)) as *const u32
}


// use std::ffi::CString;

// #[no_mangle]
// pub extern "C" fn wasm_hello(_input: *const u8, _len: usize) -> *const u32 {
//     if _input.is_null() || _len == 0 {
//         return std::ptr::null_mut();
//     }

//     let slice = unsafe { std::slice::from_raw_parts(_input, _len) };

//     let string = match std::str::from_utf8(slice) {
//         Ok(s) => s,
//         Err(_) => return std::ptr::null_mut(),
//     };

//     let string = "200".to_owned() + string;

//     let cstring = match CString::new(string.to_owned()) {
//         Ok(cstr) => cstr,
//         Err(_) => return std::ptr::null_mut(),
//     };

//     let s_len = cstring.to_bytes().len() as u32;
//     let s_ptr = cstring.into_raw() as u32;

//     let data: [u32; 2] = [s_ptr as u32, s_len as u32];

//     Box::into_raw(Box::new(data)) as *const u32
// }
