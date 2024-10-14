use std::collections::HashMap;
use std::ffi::CStr;

#[no_mangle]
extern "C" fn obtener_inventario() -> *mut HashMap<&'static str, i32> {
    let mut inventario: HashMap<&str, i32> = HashMap::new();

    inventario.insert("manzanas", 10);
    inventario.insert("peras", 5);
    inventario.insert("aguacates", 40);

    Box::into_raw(Box::new(inventario))
}

#[no_mangle]
extern "C" fn obtener_item(mapa: *mut HashMap<&'static str, i32>, key: *const c_char) -> i32 {
    let mapa = unsafe { &mut *mapa };
    let key = unsafe { CStr::from_ptr(key).to_str() }.expect("Cannot extract key value");
    let value = mapa.get(key).expect("Cannot get value");
    *value
}
