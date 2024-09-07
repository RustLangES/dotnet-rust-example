use std::mem;

#[no_mangle]
extern "C" fn get_list(out_len: *mut usize) -> *const i32 {
    let lista = vec![10, 20, 30, 40, 100];
    let len = lista.len();
    unsafe {
        *out_len = len;
    }
    let ptr = lista.as_ptr();
    mem::forget(lista);
    ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_list() {
        let mut length: usize = 0;
        let ptr = get_list(&mut length);
        let slice = unsafe { std::slice::from_raw_parts(ptr, length) };

        assert_eq!(slice, &[10, 20, 30, 40, 100]);
        assert_eq!(length, 5);
    }
}
