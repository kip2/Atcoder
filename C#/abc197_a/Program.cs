using System;

class Program
{
    static void Main()
    {
        var line = Console.ReadLine()!;

        Console.WriteLine(Solve(line));
    }

    static string Solve(string s)
    {
        string head = s.Substring(0, 1);
        string tail = s.Substring(1);
        return tail + head;
    }
}
