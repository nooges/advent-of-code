namespace AOC

open System

module Utils =

    let split c (s: string) = s.Split([| c |])
    let replace (a: string) (b: string) (s: string) = s.Replace(a, b)
    let pp a = printfn "%A" a
    let pp1 a = printfn "Part 1: %A" a
    let pp2 a = printfn "Part 2: %A" a
    let ps s = printfn "%A" (s |> Seq.toArray)
