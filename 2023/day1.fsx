#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System

let input = System.IO.File.ReadAllLines("data/day1-input.txt")

// Part 1
input
|> Array.map (fun line ->
    line.ToCharArray()
    |> Array.filter Char.IsDigit
    |> (fun ds -> 10 * (Array.head ds |> charDigitToInt) + (Array.last ds |> charDigitToInt)))
|> Array.sum
|> pp1

let digits =
    [ "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine" ]
    @ List.map string [ 1..9 ]

let findTensDigit s =
    let rec loop (s: string) pos =
        let res = digits |> List.tryFindIndex (fun d -> s.Substring(pos).StartsWith(d))

        match res with
        | Some n -> 10 * ((n % 9) + 1)
        | _ -> loop s (pos + 1)

    loop s 0

let findOnesDigit s =
    let rec loop (s: string) pos =
        let res =
            digits
            |> List.tryFindIndex (fun d -> s.Substring(s.Length - pos, pos).StartsWith(d))

        match res with
        | Some n -> (n % 9) + 1
        | _ -> loop s (pos + 1)

    loop s 1

// Part 2
input
|> Array.map (fun s -> findTensDigit s + findOnesDigit s)
|> Array.sum
|> pp2
