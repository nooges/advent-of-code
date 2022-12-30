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

let digitToSnafu =
    function
    | 3 -> "="
    | 4 -> "-"
    | d -> string d

let addDigits a b =
    match a + b with
    | 3 -> (1, 3)
    | 4 -> (1, 4)
    | 5 -> (1, 0)
    | d -> (0, d)

let snafuToBigint s =
    s
    |> asCharArray
    |> Array.rev
    |> Array.mapi (fun i c -> (pown 5I i) * (snafuToValue c))
    |> Array.sum

let bigintToDigits b source =
    let rec loop (b: int) num digits =
        let (quotient, remainder) = bigint.DivRem(num, bigint b)

        match quotient with
        | zero when zero = 0I -> int remainder :: digits
        | _ -> loop b quotient (int remainder :: digits)

    loop b source []

let bigintToSnafu (n: bigint) =

    let rec loop acc l carry =
        match (l, carry) with
        | ([], 0) -> acc
        | ([], n) -> digitToSnafu n :: acc
        | (x :: xs, carry) ->
            let (carry', d) = addDigits carry x
            loop (digitToSnafu d :: acc) xs carry'

    let b5Digits = bigintToDigits 5 n

    loop [] (b5Digits |> List.rev) 0
    |> String.concat ""


timeOperation (fun () ->
    input
    |> Array.map snafuToBigint
    |> Array.sum
    |> bigintToSnafu)
|> pp1
