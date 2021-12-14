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
    |> (printfn "%A")

let part2 =
    lines
    |> Seq.map int
    |> Seq.windowed 3
    |> Seq.map Array.sum
    |> countIncreases
    |> (printfn "%A")

//lines
//|> Seq.map int
//|> Seq.pairwise
//|> Seq.map (fun (a, b) -> a > b)
//|> Seq.iter (printfn "%A")

part1
part2
