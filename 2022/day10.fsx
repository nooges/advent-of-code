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

registerValues
|> Array.mapi (fun i v -> pixel (i % 40) v)
|> Array.chunkBySize 40
|> Array.map (String.concat "")
|> pp
