#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day10-input.txt")

let parseLine =
    function
    | "noop" -> [| 0 |]
    | line ->
        line
        |> split ' '
        |> (fun x -> [| 0; int (x[1]) |])


let registerValues =
    input
    |> Array.collect parseLine
    |> Array.scan (+) 1

[ 20; 60; 100; 140; 180; 220 ]
|> List.map (fun i -> registerValues[i - 1] * i)
|> List.sum
|> pp1
