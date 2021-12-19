#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day3-sample.txt")

let countCols acc bitList =
    Seq.zip acc bitList
    |> Seq.map (fun (a, b) -> if b = '1' then a + 1 else a)

let mostCommonBit bitsList pos =

    let count =
        bitsList
        |> Seq.map Seq.toList
        |> Seq.filter (fun a -> a.[pos] = '1')
        |> Seq.length

    if count >= (Seq.length bitsList) / 2 then
        '1'
    else
        '0'

let part1 =
    let numBits = Seq.head lines |> Seq.length

    let bitsList = lines |> Seq.map Seq.toList

    let gammaRateBitStr =
        Array.create numBits 0
        |> Seq.mapi (fun i _ -> mostCommonBit bitsList i)
        |> Array.ofSeq
        |> System.String.Concat

    let gammaRate = int ("0b" + gammaRateBitStr)
    let epsilonRate = ((1 <<< numBits) - 1) ^^^ gammaRate
    gammaRate * epsilonRate

(*
let gammaRateStr bitsList =
    let numBits = Seq.head lines |> Seq.length
    let initCounts: int [] = Array.zeroCreate numBits
    let numLines = Seq.length bitsList

    lines
    |> Seq.map Seq.toList
    |> Seq.fold countCols initCounts
    |> Seq.map (fun a -> if a > numLines / 2 then "1" else "0")
    |> String.concat ""

let gammaRateBits bitsList = gammaRateStr bitsList |> Seq.toList
*)

(*
let filterBits bitsList (criteria: list<char>) pos =
    bitsList
    |> Seq.map Seq.toList
    |> Seq.filter (fun bits -> bits.[pos] = criteria.[pos])

let rec computeRatingHelper bitsList pos =
    if Seq.length bitsList = 1 then
        Seq.head bitsList
    else
        // TODO: Make new criteria
        printfn "%A" criteria
        let filteredList = filterBits bitsList criteria pos
        printfn "%d: %A" pos filteredList
        computeRatingHelper filteredList (pos + 1)

let part2 =

    let bitsList = lines |> Seq.map Seq.toList

    let oxygenGeneratorRating = computeRatingHelper bitsList 0

    oxygenGeneratorRating
*)

//filterBits lines gammaRateBits 0

//|> Seq.iter (printfn "%A")
//|> Seq.length

printfn "Part 1: %A" part1
//printfn "Part 2: %A" part2
