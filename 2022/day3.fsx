#!/usr/bin/env -S dotnet fsi

let inputLines = System.IO.File.ReadLines("data/day3-input.txt")

let priority c = (int c - 38) % 58

let part1 =
    inputLines
    |> Seq.map (fun line ->
        line
        |> Seq.splitInto 2
        |> Seq.map Set.ofSeq
        |> Set.intersectMany
        |> Seq.head
        |> priority)
    |> Seq.sum

let part2 =
    inputLines
    |> Seq.chunkBySize 3
    |> Seq.map (fun group ->
        group
        |> Seq.map Set.ofSeq
        |> Set.intersectMany
        |> Seq.head
        |> priority)
    |> Seq.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
