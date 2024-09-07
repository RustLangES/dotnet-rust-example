using System.Runtime.InteropServices;

public static class Interop
{
    /// <summary>
    /// Llama a la función "add" desde la biblioteca "rustlib", que suma dos valores <see cref="ulong"/>
    /// <param name="a">Primer numero.</param>
    /// <param name="b">Segundo numero.</param>
    /// <returns>La suma de los dos numeros.</returns>
    [DllImport("rustlib")]
    public static extern ulong add(ulong a, ulong b);

    /// <summary>
    /// Imprime una item del enum `Frutas` en la pantalla usando una función de Rust.
    /// </summary>
    /// <param name="cosa">El item a imprimir</param>
    [DllImport("rustlib", CallingConvention = CallingConvention.Cdecl)]
    public static extern void print(Frutas cosa);

    /// <summary>
    /// Obtiene un struct `Persona`, 86 años de edad, y nacionalidad Perú.
    /// </summary>
    [DllImport("rustlib")]
    public static extern Persona get_user();

    /// <summary>
    /// Devuelve un valor del enum `Frutas` desde Rust.
    /// </summary>
    [DllImport("rustlib")]
    public static extern Frutas fruta_random();

    /// <summary>
    /// Recibe una cadena de texto desde Rust, la procesa y devuelve una nueva cadena.
    /// </summary>
    /// <param name="ola">Un puntero a una cadena de texto en memoria no gestionada.</param>
    /// <returns>Un puntero a la nueva cadena de texto generada.</returns>
    [DllImport("rustlib")]
    public static extern IntPtr texto(IntPtr ola);

    /// <summary>
    /// Libera la memoria de un string.
    /// </summary>
    /// <param name="ptr">Puntero al string a liberar.</param>
    [DllImport("rustlib")]
    public static extern void release_string(IntPtr ptr);

    /// <summary>
    /// Obten un puntero a una lista desde rust
    /// </summary>
    /// <param name="lenght">La longitud de la lista (out).</param>
    [DllImport("rustlib")]
    public static extern IntPtr get_list(out ulong lenght);

    /// <summary>
    /// Cambia la nacionalidad de una Persona a "Bolivia"
    /// </summary>
    /// <param name="persona">El objeto de `Persona` a modificar.</param>
    [DllImport("rustlib")]
    public static extern Persona cambiar_nacionalidad(Persona persona);
}
