#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day15-input.txt")

timeOperation (fun () ->
    input
    |> split ','
    |> Array.sumBy (fun s -> s |> asCharArray |> Array.fold (fun hash c -> ((hash + int c) * 17) % 256) 0))
|> pp1
