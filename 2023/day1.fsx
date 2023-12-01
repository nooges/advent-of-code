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
    |> (fun ds ->
        10 * (Array.head ds |> charDigitToInt)
        + (Array.last ds |> charDigitToInt)))
|> Array.sum
|> pp1
