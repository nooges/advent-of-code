#!/usr/bin/env -S dotnet fsi

#load "../Utils.fs"

open AOC.Utils
open System

let input = System.IO.File.ReadAllLines("input.txt")

let combos (n: int) =
    let totalAmount = 100

    let rec loop n rem =
        match (n, rem) with
        | (0, _) -> [| [ rem ] |]
        | (_, 0) -> [| [ 0 ] |]
        | _ ->
            [| 0..rem |]
            |> Array.collect (fun i -> loop (n - 1) (rem - i) |> Array.map (fun v -> i :: v))

    loop (n - 1) totalAmount

let score (ingredients: int array array) (amounts: int list) =
    amounts
    |> List.mapi (fun i amount -> ingredients[i] |> Array.map (fun a -> a * amount))
    |> List.fold (Array.map2 (+)) [| 0; 0; 0; 0; 0 |]
    |> Array.map (max 0)
    |> (fun a -> a[0] * a[1] * a[2] * a[3])

let calories (ingredients: int array array) (amounts: int list) =
    amounts |> List.mapi (fun i amount -> ingredients[i][4] * amount) |> List.sum

let ingredients = input |> Array.map allInts

// Part 1
timeOperation (fun () ->
    ingredients
    |> Array.length
    |> combos
    |> Array.map (fun a -> score ingredients a)
    |> Array.max)
|> pp1

// Part 2
timeOperation (fun () ->
    ingredients
    |> Array.length
    |> combos
    |> Array.filter (fun a -> calories ingredients a = 500)
    |> Array.map (fun a -> score ingredients a)
    |> Array.max)
|> pp2
