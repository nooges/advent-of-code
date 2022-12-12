namespace AOC

open System
open System.Text.RegularExpressions

module Utils =

    let split c (s: string) = s.Split([| c |])
    let contains (c: char) (s: string) = s.Contains(c)
    let trim (s: string) = s.Trim()
    let replace (a: string) (b: string) (s: string) = s.Replace(a, b)
    let asCharArray (s: string) = s.ToCharArray()
    let asStringArray: string [] -> string [] = Array.map string
    let asIntArray: string [] -> int [] = Array.map int
    let charToInt (c: char) = int (c) - int 'a'
    let pp a = printfn "%A" a
    let pp1 a = printfn "Part 1: %A" a
    let pp2 a = printfn "Part 2: %A" a
    let ps s = printfn "%A" (s |> Seq.toArray)

    let withRegex regex str =
        [| for m in Regex.Match(str, regex).Groups -> m.Value |]
        |> Array.tail

    type Point = { x: int; y: int }
