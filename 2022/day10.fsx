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

let pixel pos reg =
    match abs (pos - reg) <= 1 with
    | true -> "█"
    | _ -> " "

[ 20..40..240 ]
|> List.map (fun i -> registerValues[i - 1] * i)
|> List.sum
|> pp1

[ 0..239 ]
|> List.map (fun i -> pixel (i % 40) registerValues[i])
|> List.chunkBySize 40
|> List.map (String.concat "")
|> pp
