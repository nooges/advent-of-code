#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day8-input.txt")

type Note =
    { signals: string []
      outputs: string [] }

let notes =
    inputLines
    |> Seq.map (fun l -> l.Split " | ")
    |> Seq.map (fun l ->
        { signals = l.[0].Split " "
          outputs = l.[1].Split " " })

let part1 =
    notes
    |> Seq.map (fun note ->
        note.outputs
        |> Array.map (fun a -> a.Length)
        |> Array.filter (fun a -> Array.contains a [| 2; 3; 4; 7 |])
        |> Array.length)
    |> Seq.sum

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
