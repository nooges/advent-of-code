#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day13-input.txt")

let grids = input |> splitStr "\n\n" |> Array.map (splitStr "\n" >> array2D)

let findReflection grid =
    // Search rows
    let maxRow = Array2D.length1 grid - 1

    let reflectionRow =
        [ 0 .. maxRow - 1 ]
        |> List.tryFind (fun n ->
            [ 0..n ]
            |> List.map (fun m -> (n - m, n + m + 1))
            |> List.filter (fun (a, b) -> a >= 0 && b <= maxRow)
            |> List.forall (fun (a, b) -> grid[a, *] = grid[b, *]))

    // Search cols
    let maxCol = Array2D.length2 grid - 1

    let reflectionCol =
        [ 0 .. maxCol - 1 ]
        |> List.tryFind (fun n ->
            [ 0..n ]
            |> List.map (fun m -> (n - m, n + m + 1))
            |> List.filter (fun (a, b) -> a >= 0 && b <= maxCol)
            |> List.forall (fun (a, b) -> grid[*, a] = grid[*, b]))

    (reflectionRow, reflectionCol)

timeOperation (fun () ->
    grids
    |> Array.map findReflection
    |> (fun ps ->
        100 * (Array.choose fst ps |> Array.map ((+) 1) |> Array.sum)
        + (Array.choose snd ps |> Array.map ((+) 1) |> Array.sum)))
|> pp1
