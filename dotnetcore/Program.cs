// Disabled unused function warnings
#pragma warning disable CS8321 

using System.Runtime.InteropServices;


/// <summary>
/// Llama a la función "add" desde la biblioteca "rustlib", que suma dos valores <see cref="ulong"/>
/// <param name="a">Primer numero.</param>
/// <param name="b">Segundo numero.</param>
/// <returns>La suma de los dos numeros.</returns>
[DllImport("rustlib")]
static extern ulong add(ulong a, ulong b);

/// <summary>
/// Imprime una item del enum `Frutas` en la pantalla usando una función de Rust.
/// </summary>
/// <param name="cosa">El item a imprimir</param>
[DllImport("rustlib", CallingConvention = CallingConvention.Cdecl)]
static extern void print(Frutas cosa);

/// <summary>
/// Obtiene un struct `Persona`, 86 años de edad, y nacionalidad Perú.
/// </summary>
[DllImport("rustlib")]
static extern Persona get_user();

/// <summary>
/// Devuelve un valor del enum `Frutas` desde Rust.
/// </summary>
[DllImport("rustlib")]
static extern Frutas fruta_random();

/// <summary>
/// Recibe una cadena de texto desde Rust, la procesa y devuelve una nueva cadena.
/// </summary>
/// <param name="ola">Un puntero a una cadena de texto en memoria no gestionada.</param>
/// <returns>Un puntero a la nueva cadena de texto generada.</returns>
[DllImport("rustlib")]
static extern IntPtr texto(IntPtr ola);

/// <summary>
/// Libera la memoria de un string.
/// </summary>
/// <param name="ptr">Puntero al string a liberar.</param>
[DllImport("rustlib")]
static extern void release_string(IntPtr ptr);

/// <summary>
/// Obten un puntero a una lista desde rust
/// </summary>
/// <param name="lenght">La longitud de la lista (out).</param>
[DllImport("rustlib")]
static extern IntPtr get_list(out ulong lenght);


/// <summary>
/// Cambia la nacionalidad de una Persona a "Bolivia"
/// </summary>
/// <param name="persona">El objeto de `Persona` a modificar.</param>
[DllImport("rustlib")]
static extern Persona cambiar_nacionalidad(Persona persona);

void Lista() {
  IntPtr listaPtr = get_list(out ulong length);
  int[] dataArray = new int[length];
  Marshal.Copy(listaPtr, dataArray, 0, (int)length);
  foreach (var item in dataArray) {
    Console.WriteLine(item);
  }
}

/// <summary>
/// Ejemplo de uso de `texto`
/// </summary>
void Texto()
{
    IntPtr a = Marshal.StringToHGlobalAnsi("hola"); // Convierte "hola" a un puntero
    var b = texto(a);
    string result = Marshal.PtrToStringAnsi(b)!; // Convierte el puntero devuelto por "texto" a un `String`
    Console.WriteLine(result);
    release_string(b); // Libera la memoria usada por `b`
}

/// <summary>
/// Ejemplo de uso de `Persona` y `cambiar_nacionalidad`
/// </summary>
void Persona() {
  var jotchua = new Persona();
  jotchua.nombre = Marshal.StringToHGlobalAnsi("Jotchua");
  jotchua.edad = 68;
  jotchua.nacionalidad = Marshal.StringToHGlobalAnsi("Peru");

  Console.WriteLine(Marshal.PtrToStringAnsi(jotchua.nacionalidad)); // Peru
  
  cambiar_nacionalidad(jotchua);
  
  Console.WriteLine(Marshal.PtrToStringAnsi(jotchua.nacionalidad)); // Bolivia
}

[StructLayout(LayoutKind.Sequential)]
public struct Persona
{
    public IntPtr nombre;
    public uint edad;
    public IntPtr nacionalidad;
}

public enum Frutas : int
{
    Pera,
    Manzana,
    Guayaba,
    Mora,
    Aguacate,
    Tomate
}
