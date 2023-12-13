#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day13-input.txt")

let grids = input |> splitStr "\n\n" |> Array.map (splitStr "\n" >> array2D)

let reflectionPairs max n =
    [ 0 .. n - 1 ]
    |> List.map (fun m -> (n - m - 1, n + m))
    |> List.filter (fun (a, b) -> a >= 0 && b <= max)

let numDifferences a b =
    (0, a, b)
    |||> Array.fold2 (fun acc x y ->
        match x = y with
        | true -> acc
        | _ -> acc + 1)

let findReflectionValue errors grid =
    let maxRow = Array2D.length1 grid - 1

    let reflectionRowSum =
        [ 1..maxRow ]
        |> List.filter (
            reflectionPairs maxRow
            >> List.sumBy (fun (a, b) -> numDifferences grid[a, *] grid[b, *])
            >> (=) errors
        )
        |> List.sum

    let maxCol = Array2D.length2 grid - 1

    let reflectionColSum =
        [ 1..maxCol ]
        |> List.filter (
            reflectionPairs maxCol
            >> List.sumBy (fun (a, b) -> numDifferences grid[*, a] grid[*, b])
            >> (=) errors
        )
        |> List.sum

    100 * reflectionRowSum + reflectionColSum

timeOperation (fun () -> grids |> Array.sumBy (findReflectionValue 0)) |> pp1
timeOperation (fun () -> grids |> Array.sumBy (findReflectionValue 1)) |> pp2

let xx = grids[0][1, *]
