// Disabled unused function warnings
#pragma warning disable CS8321

public abstract class Program
{
    static void Main(string[] args)
    {
        var separador = String.Concat(Enumerable.Repeat("-", 20));

        // Ejemplo de uso de la función texto
        Handlers.Texto();

        Console.WriteLine(separador);

        // Ejemplo de uso de Persona y cambiar_nacionalidad
        Handlers.Persona();

        Console.WriteLine(separador);

	// Ejemplo HashMap
        Handlers.HashMap();

        Console.WriteLine(separador);

	// Ejemplo Lista
        Handlers.Lista();
    }
}
