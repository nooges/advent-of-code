#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day3-input.txt")

let countCols acc bitList =
    Seq.zip acc bitList
    |> Seq.map (fun (a, b) -> if b = '1' then a + 1 else a)

let part1 =
    let numBits = Seq.head lines |> Seq.length
    let initCounts: int [] = Array.zeroCreate numBits
    let numLines = Seq.length lines

    let gammaRateStr =
        lines
        |> Seq.map Seq.toList
        |> Seq.fold countCols initCounts
        |> Seq.map (fun a -> if a > numLines / 2 then "1" else "0")
        |> String.concat ""

    let gammaRate = int ("0b" + gammaRateStr)
    let epsilonRate = ((1 <<< numBits) - 1) ^^^ gammaRate
    gammaRate * epsilonRate

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
