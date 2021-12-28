#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day9-input.txt")

let adjacentPoints x y map =
    let maxX = map |> Array2D.length1
    let maxY = map |> Array2D.length2

    let points =
        [| (x - 1, y)
           (x + 1, y)
           (x, y - 1)
           (x, y + 1) |]

    points
    |> Array.filter (fun (x2, y2) -> x2 >= 0 && x2 < maxX && y2 >= 0 && y2 < maxY)

let isLowPoint height adjPoints (map: int [,]) =
    adjPoints
    |> Array.forall (fun (x, y) -> map.[x, y] > height)

let riskLevel height adjPoints (map: int [,]) =
    adjPoints
    |> Array.forall (fun (x, y) -> map.[x, y] > height)
    |> (fun x ->
        match x with
        | true -> height + 1
        | false -> 0)

let hmap =
    inputLines
    |> Seq.map (fun line ->
        line
        |> Seq.toList
        |> List.map string
        |> Seq.map int)
    |> array2D

let part1 =
    hmap
    |> Array2D.mapi (fun x y height -> riskLevel height (adjacentPoints x y hmap) hmap)
    |> Seq.cast<int>
    |> Seq.sum

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
