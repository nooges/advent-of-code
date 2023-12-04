#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day4-input.txt")

type Card =
    { id: int
      winners: int[]
      numbers: int[] }

// Part 1
let cardValue card =
    Set.intersect (Set.ofArray card.winners) (Set.ofArray card.numbers)
    |> (fun s -> 2.0 ** (float (s.Count - 1)) |> int)

input
|> Array.map (withRegex "Card (.*): (.*) \| (.*)")
|> Array.sumBy (fun a ->
    { id = int a[0]
      winners = extractInts a[1]
      numbers = extractInts a[2] }
    |> cardValue)
|> pp1
