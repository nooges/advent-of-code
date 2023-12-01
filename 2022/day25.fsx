#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day25-input.txt")

let snafuToValue =
    function
    | '2' -> 2I
    | '1' -> 1I
    | '0' -> 0I
    | '-' -> -1I
    | _ -> -2I

let snafuToBigint s =
    s
    |> asCharArray
    |> Array.rev
    |> Array.mapi (fun i c -> (pown 5I i) * (snafuToValue c))
    |> Array.sum

let bigintToSnafu (n: bigint) =
    let rec loop acc (n: bigint) =
        match n with
        | n when n = 0I -> acc
        | n -> loop ("012=-"[int (n % 5I)] :: acc) ((n + 2I) / 5I)

    System.String(loop [] n |> Array.ofList)

timeOperation (fun () ->
    input
    |> Array.map snafuToBigint
    |> Array.sum
    |> bigintToSnafu)
|> pp1
