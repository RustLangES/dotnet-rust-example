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
    Tomate,
}
