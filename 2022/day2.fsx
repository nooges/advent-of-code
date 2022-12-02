#!/usr/bin/env -S dotnet fsi

type Move =
    | Rock = 1
    | Paper = 2
    | Scissors = 3

type Result =
    | Win = 6
    | Draw = 3
    | Loss = 0

let codeToMove code =
    match code with
    | "A" -> Move.Rock
    | "B" -> Move.Paper
    | "C" -> Move.Scissors
    | "X" -> Move.Rock
    | "Y" -> Move.Paper
    | _ -> Move.Scissors

let strategyGuide =
    System.IO.File.ReadLines("data/day2-input.txt")
    |> Seq.map (fun line -> line.Split())
    |> Seq.map (fun line -> line |> Array.map codeToMove)

let roundResult moves =
    match moves with
    | (x, y) when x = y -> Result.Draw
    | (Move.Rock, Move.Scissors) -> Result.Loss
    | (Move.Paper, Move.Rock) -> Result.Loss
    | (Move.Scissors, Move.Paper) -> Result.Loss
    | _ -> Result.Win

let roundScore oppMove myMove =
    (unbox<int> myMove)
    + unbox<int> (roundResult (oppMove, myMove))

let part1 =
    strategyGuide
    |> Seq.map (fun line -> roundScore line[0] line[1])
    |> Seq.sum

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
