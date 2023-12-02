#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System

let input = System.IO.File.ReadAllLines("data/day1-input.txt")

// Part 1
input
|> Array.sumBy (fun s ->
    s.ToCharArray()
    |> Array.filter Char.IsDigit
    |> (fun ds -> 10 * (Array.head ds |> charDigitToInt) + (Array.last ds |> charDigitToInt)))
|> pp1

let digits =
    [ "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine" ]
    @ List.map string [ 1..9 ]

let findFirstDigit (s: string) =
    digits
    |> (List.mapi (fun i d -> (s.IndexOf(d), i % 9 + 1)))
    |> List.filter (fun (p, _) -> p >= 0)
    |> List.minBy fst
    |> snd

let findLastDigit (s: string) =
    digits
    |> (List.mapi (fun i d -> (s.LastIndexOf(d), i % 9 + 1)))
    |> List.maxBy fst
    |> snd

// Part 2
input |> Array.sumBy (fun s -> 10 * findFirstDigit s + findLastDigit s) |> pp2
