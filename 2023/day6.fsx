#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day6-input.txt")

let times = input[0] |> allInts
let distances = input[1] |> allInts

let traveled raceTime holdTime = (raceTime - holdTime) * holdTime

// Part 1
(1, times, distances)
|||> Array.fold2 (fun acc t d ->
    [| 0..t |]
    |> Array.map (traveled t)
    |> Array.filter (fun x -> x > d)
    |> Array.length
    |> (*) acc)
|> pp1

// Part 2
