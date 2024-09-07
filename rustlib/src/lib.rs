mod fruta;
mod list;
mod persona;
mod text;

#[no_mangle]
extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}
