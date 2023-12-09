#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day9-input.txt")

let diff = Array.pairwise >> Array.map (fun (a, b) -> b - a)

let nextValue =
    let rec loop acc l =
        match Array.forall ((=) 0) l with
        | true -> acc
        | _ -> loop (Array.last l + acc) (diff l)

    loop 0

timeOperation (fun () -> input |> Array.sumBy (allInts >> nextValue)) |> pp1

timeOperation (fun () -> input |> Array.sumBy (allInts >> Array.rev >> nextValue))
|> pp2
