use std::ffi::{c_char, CString};

#[no_mangle]
extern "C" fn texto(ola: *const c_char) -> *mut c_char {
    let c_texto = unsafe { CString::from_raw(ola.cast_mut()) };
    let texto = c_texto.into_string().expect("1");

    let result = format!("{texto} :3");

    let c_result = CString::new(result).expect("2");
    c_result.into_raw()
}

#[no_mangle]
extern "C" fn release_string(ptr: *mut c_char) {
    let _ = unsafe { CString::from_raw(ptr) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn text() {
        let input = CString::new("hola").unwrap();
        let ptr = texto(input.as_ptr());
        let result = unsafe { CString::from_raw(ptr).into_string().unwrap() };

        assert_eq!(result, "hola :3");
    }
}
