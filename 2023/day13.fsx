#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day13-input.txt")

let grids = input |> splitStr "\n\n" |> Array.map (splitStr "\n" >> array2D)


let reflectionPairs max n =
    [ 0 .. n - 1 ]
    |> List.map (fun m -> (n - m - 1, n + m))
    |> List.filter (fun (a, b) -> a >= 0 && b <= max)

let findReflectionValue grid =

    // Search rows
    let maxRow = Array2D.length1 grid - 1

    let reflectionRowSum =
        [ 1..maxRow ]
        |> List.filter (fun n -> reflectionPairs maxRow n |> List.forall (fun (a, b) -> grid[a, *] = grid[b, *]))
        |> List.sum

    // Search cols
    let maxCol = Array2D.length2 grid - 1

    let reflectionColSum =
        [ 1..maxCol ]
        |> List.filter (fun n -> reflectionPairs maxCol n |> List.forall (fun (a, b) -> grid[*, a] = grid[*, b]))
        |> List.sum

    100 * reflectionRowSum + reflectionColSum |>! pp

timeOperation (fun () -> grids |> Array.sumBy findReflectionValue) |> pp1
