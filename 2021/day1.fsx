#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day1-input.txt")

let countIncreases values =
    values
    |> Seq.pairwise
    |> Seq.filter (fun (a, b) -> a < b)
    |> Seq.length

let part1 =
    lines
    |> Seq.map int
    |> countIncreases

let part2 =
    lines
    |> Seq.map int
    |> Seq.windowed 3
    |> Seq.map Array.sum
    |> countIncreases

let part2NoSum =
    lines
    |> Seq.map int
    |> Seq.windowed 4
    |> Seq.filter (fun a -> (Array.head a) < (Array.last a))
    |> Seq.length

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
printfn "Part 2 (No Sum): %A" part2NoSum
