#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Text.RegularExpressions

let input = System.IO.File.ReadAllLines("data/day8-input.txt")

let moves = input[0] |> asCharArray |> Array.toList

let nodes =
    input[2..]
    |> Array.map (fun l ->
        [| for m in Regex.Matches(l, @"-?[A-Z12]+") -> m.Value |]
        |> (fun a -> (a[0], Map [ 'L', a[1]; 'R', a[2] ])))
    |> Map.ofArray

// Part 1
let traverse stopCondition start =
    let rec loop i m node =
        match m with
        | _ when stopCondition node -> i
        | [] -> loop i moves node
        | x :: xs -> loop (i + 1) xs (nodes[node][x])

    loop 0 moves start

timeOperation (fun () -> traverse (fun n -> n = "ZZZ") "AAA") |> pp1

// Part 2
let rec gcd a b =
    match () with
    | _ when b = 0I -> a
    | _ when a = 0I -> b
    | _ -> gcd b (a % b)

let lcm a b = a * b / (gcd a b)

timeOperation (fun () ->
    nodes
    |> Map.keys
    |> Seq.filter (fun n -> n[2] = 'A')
    |> Seq.map (traverse (fun n -> n[2] = 'Z') >> bigint)
    |> Seq.reduce lcm)
|> pp2
