#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Direction =
    | Left
    | Right

let input = System.IO.File.ReadAllText("data/day17-sample.txt")

let jets =
    input.ToCharArray()
    |> Array.map (fun c ->
        match c with
        | '<' -> Left
        | '>' -> Right
        | _ -> failwith "Unknown input")

let shapes =
    [| [| 0b1111 |]
       [| 0b0100; 0b1110; 0b0100 |]
       [| 0b0010; 0b0010; 0b1110 |]
       [| 0b1000; 0b1000; 0b1000 |]
       [| 0b1100; 0b1100 |] |]

let dropRock