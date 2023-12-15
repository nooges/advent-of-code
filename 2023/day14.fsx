#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day14-sample.txt")

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

let calculateNewLine (line: char array) =
    let len = line.Length

    let rec loop acc rockOffset i (l: char list) =
        match l with
        | [] ->
            let numEmpty = i + rockOffset - len
            acc + (String.replicate numEmpty ".")
        | '.' :: xs -> loop acc rockOffset (i + 1) xs // Keep going
        | 'O' :: xs -> loop (acc + "O") (rockOffset - 1) (i + 1) xs // Push onto accString
        | _ :: xs ->
            let numEmpty = i + rockOffset - len
            let acc' = acc + (String.replicate numEmpty ".") + "#"
            loop acc' (len - i - 1) (i + 1) xs // Fill rest of accString up to new offset with '.', and also push '#'

    loop "" len 0 (line |> Array.toList)

let revString s =
    new string (s |> Seq.rev |> Seq.toArray)

let tiltUpWithTranspose g =
    [ 0 .. (g |> Array2D.length2) - 1 ]
    |> List.map (fun col -> g[*, col] |> calculateNewLine)
    |> array2D

let tiltDownWithTranspose g =
    [ 0 .. (g |> Array2D.length2) - 1 ]
    |> List.map (fun col -> g[*, col] |> Array.rev |> calculateNewLine |> revString)
    |> array2D

let totalLoad g =
    [ 0 .. (g |> Array2D.length1) - 1 ]
    |> List.sumBy (fun row ->
        g[row, *]
        |> Array.filter ((=) 'O')
        |> Array.length
        |> (*) (Array2D.length2 g - row))
    |>! pp

let cycle =
    tiltUpWithTranspose
    >> tiltUpWithTranspose
    >> tiltDownWithTranspose
    >> tiltDownWithTranspose

timeOperation (fun () ->
    [ 0 .. (grid |> Array2D.length2) - 1 ]
    |> List.sumBy (fun col -> grid[*, col] |> calculateLoad))
|> pp1

let numCycles = 100

timeOperation (fun () -> [ 1..numCycles ] |> List.fold (fun g _ -> cycle g) grid |> totalLoad)
|> pp2

//cycle grid |> cycle |> pp
