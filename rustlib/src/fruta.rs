use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_print() {
        print(Frutas::Pera); // No se verifica pero debería imprimir "Pera"
    }

    #[test]
    fn test_fruta_random() {
        let frutas: Vec<Frutas> = Frutas::iter().collect();
        let fruta = fruta_random();
        assert!(frutas.contains(&fruta), "Fruta aleatoria no es válida");
    }
}
