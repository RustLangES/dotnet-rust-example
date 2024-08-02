using System.Runtime.InteropServices;

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

public abstract class Program
{
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
    [DllImport("rustlib", CallingConvention = CallingConvention.Cdecl)]
    static extern void print(Frutas cosa);

    /// <summary>
    /// Obtiene un struct `Persona` con el nombre Jotchua, 86 años de edad, y nacionalidad Perú.
    /// </summary>
    [DllImport("rustlib")]
    static extern Persona jotchua();

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

    static void Main(string[] args) { }
}
