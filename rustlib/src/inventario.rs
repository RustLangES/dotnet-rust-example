use std::collections::HashMap;
use std::ffi::{c_char, CStr};

/// Obten un inventario de productos y sus cantidades
#[no_mangle]
extern "C" fn obtener_inventario() -> *mut HashMap<&'static str, i32> {
    let mut inventario: HashMap<&str, i32> = HashMap::new();

    inventario.insert("manzanas", 10);
    inventario.insert("peras", 5);
    inventario.insert("aguacates", 40);

    Box::into_raw(Box::new(inventario))
}

#[no_mangle]
extern "C" fn obtener_cantidad(mapa: *mut HashMap<&'static str, i32>, key: *const c_char) -> i32 {
    let mapa = unsafe { &mut *mapa };
    let key = unsafe { CStr::from_ptr(key).to_str() }.expect("Cannot extract key value");

    let value = mapa.get(key).expect("Cannot get value");
    *value
}

#[no_mangle]
extern "C" fn release_inventario(mapa: *mut HashMap<&'static str, i32>) {
    if !mapa.is_null() {
        unsafe {
            mapa.drop_in_place();
        };
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn obtener() {
        let inv = obtener_inventario();
        let mapa = unsafe { &mut *inv };
        let cantidad_manzanas = mapa.get("manzanas");
        assert_eq!(cantidad_manzanas, Some(10).as_ref());

        release_inventario(inv);
    }

    #[test]
    fn cantidad() {
        let inv = obtener_inventario();
        let key = CString::new("manzanas").unwrap();
        let cant = obtener_cantidad(inv, key.as_ptr());
        assert_eq!(cant, 10);

        release_inventario(inv);
    }
}
