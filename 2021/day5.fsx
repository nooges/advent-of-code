#!/usr/bin/env -S dotnet fsi

open System.IO

type Coordinate = { x: int; y: int }

type VentLine = { c1: Coordinate; c2: Coordinate }

let inputLines = File.ReadLines("data/day5-input.txt")

let parseInputLine (str: string) =
    str.Split " -> "
    |> Array.map (fun c -> (c.Split ",") |> Array.map int)
    |> Array.map (fun c -> { x = c.[0]; y = c.[1] })
    |> (fun cs -> { c1 = cs.[0]; c2 = cs.[1] })

let isHorizontal line = line.c1.y = line.c2.y

let isVertical line = line.c1.x = line.c2.x

let isDiagonal line =
    abs (line.c1.x - line.c2.x) = abs (line.c1.y - line.c2.y)

let bounds n1 n2 = Array.sort [| n1; n2 |]

let range n1 n2 =
    if n1 < n2 then
        seq { n1..n2 }
    else
        seq { n1 .. (-1) .. n2 }

let markLineHV line (grid: int [,]) =
    if isHorizontal line then
        let b = bounds line.c1.x line.c2.x

        grid.[line.c1.y, b.[0] .. b.[1]] <-
            grid.[line.c1.y, b.[0] .. b.[1]]
            |> Array.map (fun x -> x + 1)

        grid
    elif isVertical line then
        let b = bounds line.c1.y line.c2.y

        grid.[b.[0] .. b.[1], line.c1.x] <-
            grid.[b.[0] .. b.[1], line.c1.x]
            |> Array.map (fun x -> x + 1)

        grid
    else
        grid

let markLineHVD line (grid: int [,]) =
    if isHorizontal line then
        let b = bounds line.c1.x line.c2.x

        grid.[line.c1.y, b.[0] .. b.[1]] <-
            grid.[line.c1.y, b.[0] .. b.[1]]
            |> Array.map (fun x -> x + 1)

        grid
    elif isVertical line then
        let b = bounds line.c1.y line.c2.y

        grid.[b.[0] .. b.[1], line.c1.x] <-
            grid.[b.[0] .. b.[1], line.c1.x]
            |> Array.map (fun x -> x + 1)

        grid
    elif isDiagonal line then
        let rows = range line.c1.x line.c2.x
        let cols = range line.c1.y line.c2.y

        printfn "cols: %A, row: %A" (Seq.toArray cols) (Seq.toArray rows)

        (cols, rows)
        ||> Seq.iter2 (fun x y -> grid.[x, y] <- grid.[x, y] + 1)

        grid
    else
        grid

let part1 =

    let ventLines = inputLines |> Seq.map parseInputLine |> Seq.toList
    let initialGrid = Array2D.init 1000 1000 (fun x y -> 0)
    //let initialGrid = Array2D.init 10 10 (fun x y -> 0)

    let markedGrid =
        let rec updateGrid lines grid =
            match lines with
            | [] -> grid
            | x :: xs -> updateGrid xs (markLineHV x grid)

        updateGrid ventLines initialGrid

    printfn "Grid:\n%A" markedGrid

    markedGrid
    |> Seq.cast<int>
    |> Seq.filter (fun x -> x > 1)
    |> Seq.length


let part2 =

    let ventLines = inputLines |> Seq.map parseInputLine |> Seq.toList
    let initialGrid = Array2D.init 1000 1000 (fun x y -> 0)
    //let initialGrid = Array2D.init 10 10 (fun x y -> 0)

    let markedGrid =
        let rec updateGrid lines grid =
            match lines with
            | [] -> grid
            | x :: xs -> updateGrid xs (markLineHVD x grid)

        updateGrid ventLines initialGrid

    printfn "Grid:\n%A" markedGrid

    markedGrid
    |> Seq.cast<int>
    |> Seq.filter (fun x -> x > 1)
    |> Seq.length

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
