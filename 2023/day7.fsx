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

let calculateType (cards: string) =
    let counts = cards |> asCharArray |> Array.countBy id |> Array.map snd |> Array.sort

    match counts with
    | [| 5 |] -> Type.FiveOfAKind
    | [| 1; 4 |] -> Type.FourOfAKind
    | [| 2; 3 |] -> Type.FullHouse
    | [| 1; 1; 3 |] -> Type.ThreeOfAKind
    | [| 1; 2; 2 |] -> Type.TwoPair
    | [| 1; 1; 1; 2 |] -> Type.OnePair
    | _ -> Type.HighCard

let cardStrength (c: char) =
    match c with
    | 'A' -> 14
    | 'K' -> 13
    | 'Q' -> 12
    | 'J' -> 11
    | 'T' -> 10
    | _ -> charDigitToInt c

let readHand (s: string) =
    let parts = s.Split(' ')

    { cards = parts[0] |> asCharArray |> Array.map cardStrength
      handType = calculateType parts[0]
      bid = int parts[1] }

let compareHands (a: Hand) (b: Hand) =
    match compare a.handType b.handType with
    | res when res <> 0 -> res
    | _ -> Array.compareWith compare a.cards b.cards

timeOperation (fun () ->
    input
    |> Array.map readHand
    |> Array.sortWith compareHands
    |> Array.mapi (fun i h -> (i + 1) * h.bid)
    |> Array.sum)
|> pp1
