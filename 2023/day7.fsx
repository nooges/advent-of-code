#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day7-input.txt")

type Type =
    | FiveOfAKind = 7
    | FourOfAKind = 6
    | FullHouse = 5
    | ThreeOfAKind = 4
    | TwoPair = 3
    | OnePair = 2
    | HighCard = 1

type Hand =
    { cards: int[]
      handType: Type
      bid: int }

let countsToType =
    function
    | [| 5 |] -> Type.FiveOfAKind
    | [| 4; 1 |] -> Type.FourOfAKind
    | [| 3; 2 |] -> Type.FullHouse
    | [| 3; 1; 1 |] -> Type.ThreeOfAKind
    | [| 2; 2; 1 |] -> Type.TwoPair
    | [| 2; 1; 1; 1 |] -> Type.OnePair
    | _ -> Type.HighCard

let calculateType =
    asCharArray
    >> Array.countBy id
    >> Array.map snd
    >> Array.sortDescending
    >> countsToType

let cardStrength =
    function
    | 'A' -> 14
    | 'K' -> 13
    | 'Q' -> 12
    | 'J' -> 11
    | 'T' -> 10
    | c -> charDigitToInt c

let readHand (s: string) =
    let parts = s.Split(' ')

    { cards = parts[0] |> asCharArray |> Array.map cardStrength
      handType = calculateType parts[0]
      bid = int parts[1] }

let compareHands (a: Hand) (b: Hand) =
    match compare a.handType b.handType with
    | res when res <> 0 -> res
    | _ -> Array.compareWith compare a.cards b.cards

// Part 1
timeOperation (fun () ->
    input
    |> Array.map readHand
    |> Array.sortWith compareHands
    |> Array.mapi (fun i h -> (i + 1) * h.bid)
    |> Array.sum)
|> pp1

// Part 2
let calculateJokerType (cards: string) =
    let counts = cards |> asCharArray |> Array.countBy id |> Array.sortByDescending snd

    match (counts |> Array.map fst |> Array.contains 'J', counts.Length) with
    | (false, _)
    | (_, 1) -> counts |> Array.map snd
    | _ ->
        let (jokerCount, otherCounts) = counts |> Array.partition (fst >> (=) 'J')
        let highestCount = snd otherCounts[0] + snd jokerCount[0]
        otherCounts |> Array.map snd |> Array.updateAt 0 highestCount
    |> countsToType

let readJokerHand (s: string) =
    let parts = s.Split(' ')

    { cards = parts[0] |> replace "J" "1" |> asCharArray |> Array.map cardStrength
      handType = calculateJokerType parts[0]
      bid = int parts[1] }

timeOperation (fun () ->
    input
    |> Array.map readJokerHand
    |> Array.sortWith compareHands
    |> Array.mapi (fun i h -> (i + 1) * h.bid)
    |> Array.sum)
|> pp2
