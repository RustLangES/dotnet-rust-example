using System.Runtime.InteropServices;

public abstract class Handlers
{
    public static void Lista()
    {
        IntPtr listaPtr = Interop.get_list(out ulong length);
        int[] dataArray = new int[length];
        Marshal.Copy(listaPtr, dataArray, 0, (int)length);
        foreach (var item in dataArray)
        {
            Console.WriteLine(item);
        }
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
    }

    public static void HashMap() {
      IntPtr mapa = Interop.obtener_inventario();

      string key = "aguacates";
      var keyPtr = Marshal.StringToHGlobalAnsi(key);

      var result = Interop.obtener_cantidad(mapa, keyPtr);
      Console.WriteLine($"{key}: {result}");

    }
}
