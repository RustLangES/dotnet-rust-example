using System.Runtime.InteropServices;

[DllImport("rustlib")] static extern ulong add(ulong a, ulong b);

// See https://aka.ms/new-console-template for more information
Console.WriteLine($"5 + 3 = {add(5, 3)}");
