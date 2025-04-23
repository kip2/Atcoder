using System;

class Program
{
    static void Main()
    {
        var line = Console.ReadLine();
        var parts = Array.ConvertAll(line.Split(), int.Parse);
        var a = parts[0];
        var b = parts[1];
        var c = parts[2];

        Console.WriteLine(Solve(a, b, c));
    }

    static string Solve(int a, int b, int c)
    {
        return (a * a + b * b < c * c) ? "Yes" : "No";
    }
}
