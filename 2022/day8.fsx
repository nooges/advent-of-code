#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day8-input.txt")

let grid =
    input
    |> Array.map (Seq.toArray >> Array.map (string >> int))

let grid' = grid |> Array.transpose

let isVisible x y value =
    let isMax a =
        match a with
        | [||] -> true
        | a -> Array.max a < value

    // Look left, then right, then up, then down
    (grid[y] |> Array.take x |> isMax)
    || (grid[y] |> Array.skip (x + 1) |> isMax)
    || (grid'[x] |> Array.take y |> isMax)
    || (grid'[x] |> Array.skip (y + 1) |> isMax)

let countVisible (grid: int [] []) =
    grid
    |> Array.mapi (fun y row ->
        row
        |> Array.mapi (fun x value -> isVisible x y value)
        |> Array.filter id
        |> Array.length)
    |> Array.sum

countVisible grid |> pp1
