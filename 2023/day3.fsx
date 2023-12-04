#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day3-input.txt")

// Part 1
let isSymbol (c: char) =
    (c <> '.') && not (System.Char.IsDigit(c))

let symbolLocations =
    input
    |> Array.mapi (fun y s ->
        s
        |> asCharArray
        |> Array.mapi (fun x c -> (c, { x = x; y = y }))
        |> Array.filter (fun p -> fst p |> isSymbol))
    |> Array.concat

let findNumberLocations y (s: string) =
    let rec loop acc curr pos =
        match (pos, curr) with
        | (l, "") when l = s.Length -> acc
        | (l, _) when l = s.Length -> (((string curr, { x = pos - curr.Length; y = y })) :: acc)
        | (_, "") ->
            match s[pos] with
            | c when System.Char.IsDigit(c) -> loop acc (string c) (pos + 1)
            | _ -> loop acc "" (pos + 1)
        | _ ->
            match s[pos] with
            | c when System.Char.IsDigit(c) -> loop acc (curr + (string c)) (pos + 1)
            | _ -> loop ((string curr, { x = pos - curr.Length; y = y }) :: acc) "" (pos + 1)

    loop [] "" 0

let isNear (numberLocation: string * Point) sp =
    let np = snd numberLocation
    let nlen = (fst numberLocation).Length
    abs (sp.y - np.y) <= 1 && (isBetween (np.x - 1) (np.x + nlen) sp.x)

let isPartNumber (numberLocation: string * Point) =
    symbolLocations
    |> Array.tryFind (fun sym -> snd sym |> isNear numberLocation)
    |> Option.isSome

let numberLocations =
    input |> Array.mapi (fun i s -> findNumberLocations i s) |> List.concat

numberLocations
|> List.filter isPartNumber
|> List.sumBy (fun n -> fst n |> int)
|> pp1

// Part 2
let gearRatio sp =
    numberLocations
    |> List.filter (fun n -> isNear n sp)
    |> (fun l ->
        match l.Length with
        | 2 -> (l.Head |> fst |> int) * (List.last l |> fst |> int)
        | _ -> 0)

symbolLocations
|> Array.filter (fun s -> fst s = '*')
|> Array.sumBy (snd >> gearRatio)
|> pp2
