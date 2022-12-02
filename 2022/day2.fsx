#!/usr/bin/env -S dotnet fsi

type Move =
    | Rock = 1
    | Paper = 2
    | Scissors = 3

type Result =
    | Win = 6
    | Draw = 3
    | Loss = 0

let codeToMove =
    function
    | "A" -> Move.Rock
    | "B" -> Move.Paper
    | "C" -> Move.Scissors
    | "X" -> Move.Rock
    | "Y" -> Move.Paper
    | _ -> Move.Scissors

let codeToResult =
    function
    | "X" -> Result.Loss
    | "Y" -> Result.Draw
    | _ -> Result.Win

let strategyGuide =
    System.IO.File.ReadLines("data/day2-input.txt")
    |> Seq.map (fun line -> line.Split())

let roundResult =
    function
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
    |> Seq.map (fun line -> line |> Array.map codeToMove)
    |> Seq.map (fun line -> roundScore line[0] line[1])
    |> Seq.sum

let neededMove =
    function
    | (x, Result.Draw) -> x
    | (Move.Rock, Result.Win) -> Move.Paper
    | (Move.Rock, Result.Loss) -> Move.Scissors
    | (Move.Scissors, Result.Win) -> Move.Rock
    | (Move.Scissors, Result.Loss) -> Move.Paper
    | (Move.Paper, Result.Win) -> Move.Scissors
    | (Move.Paper, Result.Loss) -> Move.Rock
    | _ -> Move.Rock

let part2 =
    strategyGuide
    |> Seq.map (fun line -> (codeToMove line[0], codeToResult line[1]))
    |> Seq.map (fun line -> roundScore (fst line) (neededMove line))
    |> Seq.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
