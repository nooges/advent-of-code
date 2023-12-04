#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day4-input.txt")

type Card =
    { id: int
      winners: int[]
      numbers: int[] }

// Part 1
let numMatches card =
    Set.intersect (Set.ofArray card.winners) (Set.ofArray card.numbers) |> Set.count

let cardValue = numMatches >> (fun s -> 2.0 ** (float (s - 1)) |> int)

let cards =
    input
    |> Array.map (withRegex "Card (.*): (.*) \| (.*)")
    |> Array.map (fun a ->
        { id = int a[0]
          winners = extractInts a[1]
          numbers = extractInts a[2] })

cards |> Array.sumBy cardValue |> pp1

// Part 2
let totalCards (matchCounts: int[]) =
    let rec loop (copies: int[]) n =
        match n with
        | n when n = matchCounts.Length -> Array.sum copies
        | _ ->
            let m = matchCounts[n]
            let cardCopies = copies[n]

            let copies' =
                Array.fold (fun c i -> c |> Array.updateAt i (c[i] + cardCopies)) copies [| (n + 1) .. (n + m) |]

            loop copies' (n + 1)

    loop (Array.create matchCounts.Length 1) 0

cards |> Array.map numMatches |> totalCards |> pp2
