#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day14-input.txt")

let grid = input |> array2D

let calculateLoad (line: char array) =
    let len = line.Length

    let rec loop acc rockOffset i (l: char list) =
        match l with
        | [] -> acc
        | '.' :: xs -> loop acc rockOffset (i + 1) xs
        | 'O' :: xs -> loop (acc + rockOffset) (rockOffset - 1) (i + 1) xs
        | _ :: xs -> loop acc (len - i - 1) (i + 1) xs

    loop 0 len 0 (line |> Array.toList)

timeOperation (fun () ->
    [ 0 .. (grid |> Array2D.length2) - 1 ]
    |> List.sumBy (fun col -> grid[*, col] |> calculateLoad))
|> pp1
