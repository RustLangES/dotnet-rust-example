use std::{mem, slice};

#[no_mangle]
extern "C" fn new_list(out_len: *mut usize) -> *const i32 {
    let lista: Vec<i32> = Vec::new();
    let len = lista.len();

    unsafe {
        *out_len = len;
    }

    let ptr = lista.as_ptr();
    mem::forget(lista);
    ptr
}

#[no_mangle]
extern "C" fn get_item(list_ptr: *mut i32, len: usize, index: usize) -> i32 {
    assert!(index < len, "Index out of bounds");
    let slice = unsafe { slice::from_raw_parts(list_ptr, len) };
    let item = slice[index];

    item
}

#[no_mangle]
extern "C" fn add_item(
    list_ptr: *mut i32,
    len: usize,
    item: i32,
    out_len: *mut usize,
) -> *const i32 {
    let mut vec = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
    vec.push(item);

    unsafe {
        *out_len = vec.len();
    };

    let new_ptr = vec.as_ptr();
    mem::forget(vec);
    new_ptr
}

#[no_mangle]
extern "C" fn remove_item(
    list_ptr: *mut i32,
    len: usize,
    index: usize,
    out_len: *mut usize,
) -> *const i32 {
    let mut vec = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
    vec.remove(index);

    unsafe {
        *out_len = vec.len();
    };

    let new_ptr = vec.as_ptr();
    mem::forget(vec);
    new_ptr
}

#[no_mangle]
extern "C" fn release_list(list_ptr: *mut i32, len: usize) {
    _ = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
}

#[cfg(test)]
mod tests {}
