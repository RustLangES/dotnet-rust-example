use std::alloc::{dealloc, Layout};
use std::ffi::{c_char, CStr, CString};

fn str_to_char(text: &str) -> *mut c_char {
    let c_texto = CString::new(text).expect("Failed to convert");
    c_texto.into_raw()
}

#[repr(C)]
struct Persona {
    nombre: *mut c_char,
    edad: u32,
    nacionalidad: *mut c_char,
}

#[no_mangle]
extern "C" fn get_user() -> Persona {
    Persona {
        nombre: str_to_char("jotchua"),
        edad: 86,
        nacionalidad: str_to_char("Peru"),
    }
}

#[no_mangle]
extern "C" fn cambiar_nacionalidad(persona: &mut Persona) -> &mut Persona {
    let nacionalidad_actual = unsafe { CStr::from_ptr(persona.nacionalidad) };
    let layout = Layout::from_size_align(
        nacionalidad_actual.to_bytes().len(),
        std::mem::align_of::<u8>(),
    )
    .unwrap();

    unsafe {
        dealloc(persona.nacionalidad.cast::<u8>(), layout);
    };
    let nueva_nacionalidad = CString::new("Bolivia").expect(".");
    persona.nacionalidad = nueva_nacionalidad.into_raw();
    persona
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user() {
        let user = get_user();
        let nombre = unsafe { CString::from_raw(user.nombre).into_string().unwrap() };
        let nacionalidad = unsafe { CString::from_raw(user.nacionalidad).into_string().unwrap() };

        assert_eq!(nombre, "jotchua");
        assert_eq!(user.edad, 86);
        assert_eq!(nacionalidad, "Peru");
    }

    #[test]
    fn test_cambiar_nacionalidad() {
        let mut user = &mut get_user();
        user = cambiar_nacionalidad(user);
        let nacionalidad = unsafe { CString::from_raw(user.nacionalidad).into_string().unwrap() };

        assert_eq!(nacionalidad, "Bolivia");
    }
}