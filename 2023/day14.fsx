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

let cycle =
    tiltUpWithTranspose
    >> tiltUpWithTranspose
    >> tiltDownWithTranspose
    >> tiltDownWithTranspose

timeOperation (fun () ->
    [ 0 .. (grid |> Array2D.length2) - 1 ]
    |> List.sumBy (fun col -> grid[*, col] |> calculateLoad))
|> pp1

// Run enough cycles to determine when pattern repeats
let numCycles = 200

let findCycleLen loads =
    // Check a few values in the cycle, because some values can be repeated
    [ 0..2 ]
    |> List.map (fun i ->
        let xs = loads |> List.skip i
        let lastLoad = xs |> List.head

        xs
        |> List.tail
        |> List.find (fun l -> (snd l) = (snd lastLoad))
        |> fst
        |> (-) (fst lastLoad))
    |> List.max

timeOperation (fun () ->
    let rec loop acc n g =
        match n with
        | n when n > numCycles -> acc
        | _ ->
            let g' = cycle g
            (n, totalLoad g') |> pp

            loop ((n, totalLoad g') :: acc) (n + 1) g'

    let loads = loop [] 1 grid
    let cycleLen = findCycleLen loads
    let offset = ((1_000_000_000 - numCycles) % cycleLen) - cycleLen + numCycles
    loads |> List.find (fun l -> (fst l) = offset) |> snd)
|> pp2
