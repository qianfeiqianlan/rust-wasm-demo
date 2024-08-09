use std::ffi::CString;

#[no_mangle]
pub extern "C" fn plugin_name(_input: *const u8, _len: usize) -> *const u32 {
    let cstring = match CString::new("hello world!".to_owned()) {
        Ok(cstr) => cstr,
        Err(_) => return std::ptr::null_mut(),
    };

    let s_len = cstring.to_bytes().len() as u32;
    let s_ptr = cstring.into_raw() as u32;

    let data: [u32; 2] = [s_ptr as u32, s_len as u32];

    Box::into_raw(Box::new(data)) as *const u32
}
