#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Text.RegularExpressions

let input = System.IO.File.ReadAllLines("data/day8-input.txt")

type Node = { left: string; right: string }

let moves = input[0] |> asCharArray |> Array.toList

let nodes =
    input[2..]
    |> Array.map (fun l ->
        [| for m in Regex.Matches(l, @"-?[A-Z]+") -> m.Value |]
        |> (fun a -> (a[0], Map [ 'L', a[1]; 'R', a[2] ])))
    |> Map.ofArray

// Part 1
let traverse start =
    let rec loop i (m: list<char>) node =
        match (node, m) with
        | ("ZZZ", _) -> i
        | (_, []) -> loop i moves node
        | (_, x :: xs) -> loop (i + 1) xs (nodes[node][x])

    loop 0 moves start

timeOperation (fun () -> traverse "AAA") |> pp1
