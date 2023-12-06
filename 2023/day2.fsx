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

let isGamePossible =
    split ':'
    >> Array.last
    >> split ';'
    >> Array.forall (parseCubeSet >> Array.forall isCubeColorPossible)

input
|> Array.mapi (fun i s ->
    match (s |> isGamePossible) with
    | true -> i + 1
    | _ -> 0)
|> Array.sum
|> pp1

// Part 2
let gamePowerSet =
    split ':'
    >> Array.last
    >> split ';'
    >> Array.map (parseCubeSet)
    >> Array.concat
    >> Array.groupBy snd
    >> Array.map (snd >> Array.maxBy fst >> fst)
    >> Array.reduce (*)

input |> Array.map gamePowerSet |> Array.sum |> pp2
