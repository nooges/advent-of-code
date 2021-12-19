#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day3-input.txt")

let countCols acc bitList =
    Seq.zip acc bitList
    |> Seq.map (fun (a, b) -> if b = '1' then a + 1 else a)

let mostCommonBit bitsList pos =

    let count =
        bitsList
        |> Seq.map Seq.toList
        |> Seq.filter (fun a -> a.[pos] = '1')
        |> Seq.length

    if count >= (Seq.length bitsList - count) then
        '1'
    else
        '0'

let leastCommonBit bitsList pos =

    let count =
        bitsList
        |> Seq.map Seq.toList
        |> Seq.filter (fun a -> a.[pos] = '0')
        |> Seq.length

    if count <= (Seq.length bitsList - count) then
        '0'
    else
        '1'

let bitlistToInt (bitList: seq<char>) =
    let bitStr =
        bitList |> Array.ofSeq |> System.String.Concat

    int ("0b" + bitStr)

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

let filterBits bitsList (criteria: list<char>) pos =
    bitsList
    |> Seq.map Seq.toList
    |> Seq.filter (fun bits -> bits.[pos] = criteria.[pos])

let printBitslist b = b |> Seq.iter (printfn "%A")

let rec computeRatingHelper bitsList f pos =
    if Seq.length bitsList = 1 then
        Seq.head bitsList
    else
        let criteria = f bitsList pos
        printfn "Criteria (%d): %A" pos criteria

        let filteredList =
            bitsList
            |> Seq.map Seq.toList
            |> Seq.filter (fun bits -> bits.[pos] = criteria)

        printBitslist filteredList

        computeRatingHelper filteredList f (pos + 1)

let part2 =

    let bitsList = lines |> Seq.map Seq.toList

    let oxygenGeneratorRating =
        computeRatingHelper bitsList mostCommonBit 0
        |> bitlistToInt

    let co2ScrubberRating =
        computeRatingHelper bitsList leastCommonBit 0
        |> bitlistToInt

    oxygenGeneratorRating * co2ScrubberRating

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
