#![warn(clippy::pedantic)]

mod fruta;
mod inventario;
mod lista;
mod persona;
mod text;

#[no_mangle]
extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
