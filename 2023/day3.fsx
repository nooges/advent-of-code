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
        | (l, n) when l = s.Length -> (((string curr, { x = pos - curr.Length; y = y })) :: acc)
        | (_, "") ->
            match s[pos] with
            | c when System.Char.IsDigit(c) -> loop acc (string c) (pos + 1)
            | _ -> loop acc "" (pos + 1)
        | _ ->
            match s[pos] with
            | c when System.Char.IsDigit(c) -> loop acc (curr + (string c)) (pos + 1)
            | _ -> loop ((string curr, { x = pos - curr.Length; y = y }) :: acc) "" (pos + 1)

    loop [] "" 0

let isPartNumber (numberLocation: string * Point) =
    let np = snd numberLocation
    let nlen = (fst numberLocation).Length

    let isNear sp =
        abs (sp.y - np.y) <= 1 && (isBetween (np.x - 1) (np.x + nlen) sp.x)

    symbolLocations |> Array.map (fun sym -> snd sym |> isNear) |> Array.exists id

let numberLocations =
    input
    |> Array.mapi (fun i s -> findNumberLocations i s |> List.filter isPartNumber)
    |> List.concat
    |> List.sumBy (fun n -> fst n |> int)
    |> pp1
