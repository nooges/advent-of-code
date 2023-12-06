#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day6-input.txt")

let times = input[0] |> allInt64s
let distances = input[1] |> allInt64s

let traveled raceTime holdTime = (raceTime - holdTime) * holdTime

// Part 1
(1, times, distances)
|||> Array.fold2 (fun acc t d ->
    [| 1L .. t |]
    |> Array.filter (fun i -> (traveled t i) > d)
    |> Array.length
    |> (*) acc)
|> pp1

// Part 2
let t2 = input[0].Replace(" ", "") |> allInt64s |> Array.head
let d2 = input[1].Replace(" ", "") |> allInt64s |> Array.head

timeOperation (fun () ->
    let start = seq { 1L .. t2 } |> Seq.find (fun n -> traveled t2 n > d2)
    let stop = seq { 1L .. t2 } |> Seq.findBack (fun n -> traveled t2 n > d2)
    stop - start + 1L)
|> pp2
