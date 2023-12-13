#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day13-input.txt")

let grids = input |> splitStr "\n\n" |> Array.map (splitStr "\n" >> array2D)


let reflectionPairs max n =
    [ 0 .. n - 1 ]
    |> List.map (fun m -> (n - m - 1, n + m))
    |> List.filter (fun (a, b) -> a >= 0 && b <= max)

let findReflection grid =

    // Search rows
    let maxRow = Array2D.length1 grid - 1

    let reflectionRow =
        [ 1..maxRow ]
        |> List.tryFind (fun n -> reflectionPairs maxRow n |> List.forall (fun (a, b) -> grid[a, *] = grid[b, *]))

    // Search cols
    let maxCol = Array2D.length2 grid - 1

    let reflectionCol =
        [ 1..maxCol ]
        |> List.tryFind (fun n -> reflectionPairs maxCol n |> List.forall (fun (a, b) -> grid[*, a] = grid[*, b]))

    (reflectionRow, reflectionCol) |>! pp

timeOperation (fun () ->
    grids
    |> Array.map findReflection
    |> (fun ps -> 100 * (Array.choose fst ps |> Array.sum) + (Array.choose snd ps |> Array.sum)))
|> pp1
