open System

let solve a b c =
    if a * a + b * b < c * c then "Yes" else "No"

[<EntryPoint>]
let main _ =
    let line = Console.ReadLine()
    let parts = line.Split() |> Array.map int
    let a, b, c = parts.[0], parts.[1], parts.[2]
    solve a b c |> printfn "%s"
    0
