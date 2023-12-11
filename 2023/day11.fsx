#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day11-input.txt")

let grid = input |> array2D

let emptyRows =
    [ 0 .. (grid |> Array2D.length1) - 1 ]
    |> List.filter (fun r -> not (Array.contains '#' grid[r, *]))

let emptyCols =
    [ 0 .. (grid |> Array2D.length2) - 1 ]
    |> List.filter (fun c -> not (Array.contains '#' grid[*, c]))

emptyRows |> pp
emptyCols |> pp

let distance a b =
    let space =
        (emptyRows |> List.filter (isBetween (min a.r b.r) (max a.r b.r)) |> List.length)
        + (emptyCols |> List.filter (isBetween (min a.c b.c) (max a.c b.c)) |> List.length)

    abs (a.r - b.r) + abs (a.c - b.c) + space

let galaxies =
    [ 0 .. (grid |> Array2D.length1) - 1 ]
    |> List.collect (fun r ->
        [ 0 .. (grid |> Array2D.length2) - 1 ]
        |> List.map (fun c -> ({ r = r; c = c }, grid[r, c]))
        |> List.filter (fun (_, c) -> c = '#'))
    |> List.map fst

let pairDistances points =
    let rec loop acc l =
        match l with
        | [] -> acc
        | x :: xs -> loop (acc + (xs |> List.sumBy (distance x))) xs

    loop 0 points

timeOperation (fun () -> pairDistances galaxies) |> pp1
