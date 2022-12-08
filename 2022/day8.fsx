#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day8-input.txt")

let grid =
    input
    |> Array.map (Seq.toArray >> Array.map (string >> int))

let grid' = grid |> Array.transpose

let lookLeft x y = grid[y] |> Array.take x |> Array.rev
let lookRight x y = grid[y] |> Array.skip (x + 1)
let lookUp x y = grid'[x] |> Array.take y |> Array.rev
let lookDown x y = grid'[x] |> Array.skip (y + 1)

let lookAround =
    [| lookLeft
       lookRight
       lookUp
       lookDown |]

let isVisible x y value =
    let isMax a =
        match a with
        | [||] -> true
        | a -> Array.max a < value

    lookAround
    |> Array.map (fun f -> f x y |> isMax)
    |> Array.contains true

let countVisible (grid: int [] []) =
    grid
    |> Array.mapi (fun y row ->
        row
        |> Array.mapi (fun x value -> isVisible x y value)
        |> Array.filter id
        |> Array.length)
    |> Array.sum

let scenicScore x y value =
    let numViewable a =
        match a with
        | [||] -> 0
        | a when Array.max a < value -> a.Length
        | _ -> (Array.findIndex (fun x -> x >= value) a) + 1

    lookAround
    |> Array.map (fun f -> f x y |> numViewable)
    |> Array.reduce (*)

let highestScenicScore (grid: int [] []) =
    grid
    |> Array.mapi (fun y row ->
        row
        |> Array.mapi (fun x value -> scenicScore x y value)
        |> Array.max)
    |> Array.max

countVisible grid |> pp1
highestScenicScore grid |> pp2
