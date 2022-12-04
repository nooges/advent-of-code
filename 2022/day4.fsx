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
    (a[0] <= b[0] && a[1] >= b[1])
    || (b[0] <= a[0] && b[1] >= a[1])

let rangesOverlap (a: int []) (b: int []) =
    (a[0] <= b[0] && b[0] <= a[1])
    || (b[0] <= a[0] && a[0] <= b[1])

let part1 =
    assignments
    |> Seq.filter (fun x -> containsRange x[0] x[1])
    |> Seq.length

let part2 =
    assignments
    |> Seq.filter (fun x -> rangesOverlap x[0] x[1])
    |> Seq.length

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
