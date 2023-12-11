#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day10-input.txt")

// Map of look directions
let pipePaths =
    Map [ 'F', [ (0, 1); (1, 0) ]
          'L', [ (0, 1); (-1, 0) ]
          'J', [ (0, -1); (-1, 0) ]
          '7', [ (0, -1); (1, 0) ]
          '|', [ (1, 0); (-1, 0) ]
          '-', [ (0, 1); (0, -1) ] ]

let grid = input |> array2D

let startRow =
    [ 0 .. (grid |> Array2D.length1) - 1 ]
    |> List.find (fun r -> grid[r, *] |> Array.contains 'S')

let startCol =
    [ 0 .. (grid |> Array2D.length2) - 1 ]
    |> List.find (fun c -> grid[*, c] |> Array.contains 'S')

let start = { r = startRow; c = startCol }
start |> pp

let nextPoint pipeType curr prev =
    pipePaths[pipeType]
    |> List.map (fun offset ->
        { r = curr.r + fst offset
          c = curr.c + snd offset })
    |> List.find (fun p -> p <> prev)

let rec traverse (traveled: GridPoint list) =
    let curr = List.head traveled

    match grid[curr.r, curr.c] with
    | 'S' -> traveled
    | p ->
        let next = nextPoint p curr (traveled |> List.tail |> List.head)
        traverse (next :: traveled)

let firstNextPoint =
    if "|F7".Contains grid[start.r - 1, start.c] then
        { r = start.r - 1; c = start.c }
    elif "|LJ".Contains grid[start.r + 1, start.c] then
        { r = start.r + 1; c = start.c }
    elif "-FL".Contains grid[start.r, start.c - 1] then
        { r = start.r; c = start.c - 1 }
    else
        { r = start.r; c = start.c + 1 }

timeOperation (fun () ->
    traverse [ firstNextPoint; start ]
    |> List.length
    |> (fun n -> (n - 1) / 2))
|> pp1

let loopPoints = traverse [ firstNextPoint; start ]

// Manually replacing the pipe type for S, even though I could write code to figure it out
Array2D.set grid startRow startCol '|'

let countEnclosedPoints (row: int) (chars: char array) =
    let rec loop acc flag col =
        match col < chars.Length with
        | false -> acc
        | _ ->
            match (loopPoints |> List.contains ({ r = row; c = col }), "F7|".Contains grid[row, col], flag) with
            | (true, true, _) -> loop acc (not flag) (col + 1)
            | (false, _, true) -> loop (acc + 1) flag (col + 1)
            | _ -> loop acc flag (col + 1)

    loop 0 false 0

timeOperation (fun () ->
    [ 0 .. (grid |> Array2D.length1) - 1 ]
    |> List.sumBy (fun r -> countEnclosedPoints r grid[r, *]))
|> pp2
