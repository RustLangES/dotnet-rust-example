using System.Runtime.InteropServices;

public abstract class Handlers
{
    public static void Lista()
    {
        ulong length;
        IntPtr listPtr = Interop.new_list(out length);

        Console.WriteLine($"Nueva lista con tamaño: {length}");

        listPtr = Interop.add_item(listPtr, length, 42, out length);
        Console.WriteLine($"Añadido `42` a lista, nuevo tamaño : {length}");

        int item = Interop.get_item(listPtr, length, 0);
        Console.WriteLine($"Item en indice 0: {item}");

        listPtr = Interop.remove_item(listPtr, length, 0, out length);
        Console.WriteLine($"Eliminado item en 0, nuevo tamaño: {length}");

        Interop.release_list(listPtr);
    }

    /// <summary>
    /// Ejemplo de uso de `texto`
    /// </summary>
    public static void Texto()
    {
        IntPtr a = Marshal.StringToHGlobalAnsi("hola"); // Convierte "hola" a un puntero
        var b = Interop.texto(a);
        string result = Marshal.PtrToStringAnsi(b)!; // Convierte el puntero devuelto por "texto" a un `String`
        Console.WriteLine(result);
        Interop.release_string(b); // Libera la memoria usada por `b`
    }

    /// <summary>
    /// Ejemplo de uso de `Persona` y `cambiar_nacionalidad`
    /// </summary>
    public static void Persona()
    {
        var jotchua = new Persona();
        jotchua.nombre = Marshal.StringToHGlobalAnsi("Jotchua");
        jotchua.edad = 68;
        jotchua.nacionalidad = Marshal.StringToHGlobalAnsi("Peru");

        Console.WriteLine(Marshal.PtrToStringAnsi(jotchua.nacionalidad)); // Peru

        Interop.cambiar_nacionalidad(jotchua);

        Console.WriteLine(Marshal.PtrToStringAnsi(jotchua.nacionalidad)); // Bolivia

        Interop.release_persona(jotchua);
    }

    public static void HashMap()
    {
        IntPtr mapa = Interop.obtener_inventario();

        string key = "aguacates";
        var keyPtr = Marshal.StringToHGlobalAnsi(key);

        var result = Interop.obtener_cantidad(mapa, keyPtr);
        Console.WriteLine($"{key}: {result}");

        Interop.release_inventario(mapa);
    }
}
