use std::{mem, slice};

#[no_mangle]
extern "C" fn new_list(out_len: *mut usize) -> *mut i32 {
    let mut lista: Vec<i32> = Vec::new();
    let len = lista.len();

    unsafe {
        *out_len = len;
    }

    let ptr = lista.as_mut_ptr();
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
extern "C" fn add_item(list_ptr: *mut i32, len: usize, item: i32, out_len: *mut usize) -> *mut i32 {
    let mut vec = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
    vec.push(item);

    unsafe {
        *out_len = vec.len();
    };

    let new_ptr = vec.as_mut_ptr();
    mem::forget(vec);
    new_ptr
}

#[no_mangle]
extern "C" fn remove_item(
    list_ptr: *mut i32,
    len: usize,
    index: usize,
    out_len: *mut usize,
) -> *mut i32 {
    let mut vec = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
    vec.remove(index);

    unsafe {
        *out_len = vec.len();
    };

    let new_ptr = vec.as_mut_ptr();
    mem::forget(vec);
    new_ptr
}

#[no_mangle]
extern "C" fn release_list(list_ptr: *mut i32, len: usize) {
    if !list_ptr.is_null() {
        _ = unsafe { Vec::from_raw_parts(list_ptr, len, len) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut len = 0;
        let list_ptr = new_list(&mut len);

        assert_eq!(len, 0);
        assert!(!list_ptr.is_null());
    }

    #[test]
    fn get_add() {
        let mut len = 0;
        let list_ptr = new_list(&mut len as *mut usize);

        let mut new_len = 0;
        let list_ptr = add_item(list_ptr, len, 10, &mut new_len as *mut usize);

        assert_eq!(new_len, 1);
        let item = get_item(list_ptr, new_len, 0);
        assert_eq!(item, 10);
    }

    #[test]
    fn remove() {
        let mut len = 0;
        let list_ptr = new_list(&mut len as *mut usize);

        let mut new_len = 0;
        let list_ptr = add_item(list_ptr, len, 10, &mut new_len as *mut usize);
        let list_ptr = add_item(list_ptr, new_len, 20, &mut new_len as *mut usize);

        assert_eq!(new_len, 2);
        let list_ptr = remove_item(list_ptr, new_len, 0, &mut new_len as *mut usize);

        assert_eq!(new_len, 1);
        let item = get_item(list_ptr, new_len, 0);
        assert_eq!(item, 20);
    }
}
