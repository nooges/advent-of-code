#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Text.RegularExpressions

let input = System.IO.File.ReadAllLines("data/day9-input.txt")

let diff l =
    l |> Array.pairwise |> Array.map (fun (a, b) -> b - a)

let repeatDiff l =
    let rec loop acc i s =
        match Array.forall ((=) 0) s with
        | true -> acc
        | _ -> loop (s :: acc) (i + 1) (diff s)

    loop [] 0 l |> List.sumBy Array.last

timeOperation (fun () -> input |> Array.sumBy (allInts >> repeatDiff)) |> pp1

timeOperation (fun () -> input |> Array.sumBy (allInts >> Array.rev >> repeatDiff))
|> pp2
