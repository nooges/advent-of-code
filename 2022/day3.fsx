#!/usr/bin/env -S dotnet fsi

let inputLines = System.IO.File.ReadLines("data/day3-input.txt")

let priority =
    function
    | c when int c > 96 -> int c - 96
    | c -> int c - 38

let part1 =
    inputLines
    |> Seq.map (fun line ->
        line
        |> Seq.toArray
        |> Array.splitInto 2
        |> Array.map Set.ofArray
        |> Set.intersectMany
        |> Set.toList
        |> List.head
        |> priority)
    |> Seq.sum

let part2 =
    inputLines
    |> Seq.chunkBySize 3
    |> Seq.map (fun group ->
        group
        |> Seq.map Set.ofSeq
        |> Set.intersectMany
        |> Set.toList
        |> List.head
        |> priority)
    |> Seq.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
