#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System

let input = System.IO.File.ReadAllLines("data/day2-input.txt")

// Part 1
let cubeLimits = Map [ "red", 12; "green", 13; "blue", 14 ]

let isCubeColorPossible (p: int * string) = cubeLimits[snd p] >= fst p

let parseCubeSet =
    trim
    >> split ','
    >> Array.map (trim >> split ' ' >> (fun p -> (int p[0], p[1])))

let parseGame =
    split ':'
    >> Array.last
    >> split ';'
    >> Array.forall (parseCubeSet >> Array.forall isCubeColorPossible)

input
|> Array.mapi (fun i s ->
    let res = s |> parseGame

    match res with
    | true -> i + 1
    | _ -> 0)
|> Array.sum
|> pp1
