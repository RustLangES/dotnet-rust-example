<div align="right">
<a href="./README_EN.md">EN</a>
</div>

# Ejemplo de interoperabilidad entre DotNet y Rust

## Features
- Entorno Nix para trabajar comodo

### Requisitos

Para construir y desplegar este proyecto, necesitarás lo siguiente:

- [Rust](https://rust-lang.org)
- [dotnet](https://dotnet.microsoft.com/en-us/download)

### Aprende Rust
Contamos con recursos que pueden guiarte en tu proceso de aprendizaje de Rust

- [Recursos y colleciones de aprendizaje](https://rustlang-es.org/aprende)
- [Libro oficial en Español](https://book.rustlang-es.org)
- [Guia de Rust para Desarrolladores DotNet](https://dotnet-book.rustlang-es.org)

### Cosas a tener en cuenta
Los tipos en la interoperabilidad son importantes, por eso revisa esta tabla de equivalentes

> [!IMPORTANT]
> Mas informacion en [este capitulo del libro](https://dotnet-book.rustlang-es.org/language/scalar-types)

La siguiente tabla enumera los tipos primitivos en Rust y su equivalente en
C# y .NET:

| Rust    | C#        | .NET                   |
| ------- | --------- | ---------------------- |
| `bool`  | `bool`    | `Boolean`              |
| `char`  | `char`    | `Char`                 |
| `i8`    | `sbyte`   | `SByte`                |
| `i16`   | `short`   | `Int16`                |
| `i32`   | `int`     | `Int32`                |
| `i64`   | `long`    | `Int64`                |
| `i128`  |           | `Int128`               |
| `isize` | `nint`    | `IntPtr`               |
| `u8`    | `byte`    | `Byte`                 |
| `u16`   | `ushort`  | `UInt16`               |
| `u32`   | `uint`    | `UInt32`               |
| `u64`   | `ulong`   | `UInt64`               |
| `u128`  |           | `UInt128`              |
| `usize` | `nuint`   | `UIntPtr`              |
| `f32`   | `float`   | `Single`               |
| `f64`   | `double`  | `Double`               |
|         | `decimal` | `Decimal`              |
| `()`    | `void`    | `Void` o `ValueTuple`  |
|         | `object`  | `Object`               |

> [!IMPORTANT]
> Mas informacion en [este capitulo del libro](https://dotnet-book.rustlang-es.org/language/strings)

La comparación de textos es mostrada en la siguiente tabla:

| Rust               | .NET                 |
| ------------------ | -------------------- |
| `&mut str`         | `Span<char>`         |
| `&str`             | `ReadOnlySpan<char>` |
| `Box<str>`         | `String`             |
| `String`           | `String`             |
| `String` (mutable) | `StringBuilder`      |

> [!IMPORTANT]
> Mas informacion en [este capitulo del libro](https://dotnet-book.rustlang-es.org/language/structured-types)

La comparativa de algunos tipos de datos complejos se refleja en la siguiente tabla:
| C#           | Rust      |
| ------------ | --------- |
| `Array`      | `Array`   |
| `List`       | `Vec`     |
| `Tuple`      | `Tuple`   |
| `Dictionary` | `HashMap` |
