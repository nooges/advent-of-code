#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let assignments =
    System.IO.File.ReadLines("data/day4-input.txt")
    |> Seq.map (fun line ->
        line
        |> split ','
        |> Array.map (split '-' >> Array.map int))

let containsRange (a: int []) (b: int []) =
    match (a, b) with
    | (x, y) when x[0] <= y[0] && x[1] >= y[1] -> true
    | (y, x) when x[0] <= y[0] && x[1] >= y[1] -> true
    | _ -> false

let rangesOverlap (a: int []) (b: int []) =
    match (a, b) with
    | (x, y) when x[0] <= y[0] && y[0] <= x[1] -> true
    | (y, x) when x[0] <= y[0] && y[0] <= x[1] -> true
    | _ -> false

let part1 =
    assignments
    |> Seq.map (fun x -> containsRange x[0] x[1])
    |> Seq.filter id
    |> Seq.length

let part2 =
    assignments
    |> Seq.map (fun x -> rangesOverlap x[0] x[1])
    |> Seq.filter id
    |> Seq.length

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
