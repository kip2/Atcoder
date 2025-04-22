open System

let solve (s: string) =
  match Seq.toList s with
  | [] -> ""
  | x::xs -> (xs @ [x]) |> System.String.Concat

[<EntryPoint>]
let main _ =
    let s = Console.ReadLine()
    solve s |> printfn "%s"
    0
