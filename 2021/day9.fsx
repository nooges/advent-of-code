#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day9-sample.txt")

let hmap =
    inputLines
    |> Seq.map (fun line ->
        line
        |> Seq.toList
        |> List.map string
        |> Seq.map int)
    |> array2D

let part1 = hmap
let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
