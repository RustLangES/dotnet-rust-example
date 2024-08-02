use rand::seq::SliceRandom;
use std::ffi::CString;
use std::os::raw::c_char;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[no_mangle]
extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[repr(C)]
struct Persona {
    nombre: *mut c_char,
    edad: u32,
    nacionalidad: *mut c_char,
}

/// Convierte un `&str` a un puntero `*mut c_char`.
fn str_to_char(text: &str) -> *mut c_char {
    let c_texto = CString::new(text).expect("Failed to convert");
    c_texto.into_raw()
}

#[no_mangle]
extern "C" fn get_user() -> Persona {
    Persona {
        nombre: str_to_char("jotchua"),
        edad: 86,
        nacionalidad: str_to_char("Peru"),
    }
}

#[derive(Debug, Clone, EnumIter)]
#[repr(C)]
enum Frutas {
    Pera,
    Manzana,
    Guayaba,
    Mora,
    Aguacate,
    Tomate,
}

#[no_mangle]
extern "C" fn print(cosa: Frutas) {
    println!("{cosa:?}");
}

#[no_mangle]
extern "C" fn fruta_random() -> Frutas {
    let a: Vec<Frutas> = Frutas::iter().collect();
    let mut rng = rand::thread_rng();
    a.choose(&mut rng).unwrap().clone()
}

/// Combina una cadena C recibida con un sufijo y devuelve el resultado como un puntero `*mut c_char`.
#[no_mangle]
extern "C" fn texto(ola: *const c_char) -> *mut c_char {
    let c_texto = unsafe { CString::from_raw(ola.cast_mut()) };
    let texto = c_texto.into_string().expect("1");

    let result = format!("{texto} :3");

    let c_result = CString::new(result).expect("2");
    c_result.into_raw()
}
