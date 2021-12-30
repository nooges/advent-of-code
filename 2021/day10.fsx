#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day10-input.txt")

type BracketMode =
    | Open
    | Close

type BracketStyle =
    | Round
    | Square
    | Curly
    | Angle
    | None

type Bracket =
    { mode: BracketMode
      style: BracketStyle }

let parse c =
    match c with
    | '(' -> { mode = Open; style = Round }
    | '[' -> { mode = Open; style = Square }
    | '{' -> { mode = Open; style = Curly }
    | '<' -> { mode = Open; style = Angle }
    | ')' -> { mode = Close; style = Round }
    | ']' -> { mode = Close; style = Square }
    | '}' -> { mode = Close; style = Curly }
    | '>' -> { mode = Close; style = Angle }
    | _ -> { mode = Close; style = Angle }

let score s =
    match s with
    | Round -> 3
    | Square -> 57
    | Curly -> 1197
    | Angle -> 25137
    | None -> 0

let checkLine line =
    let brackets = line |> Seq.toList |> List.map parse

    let rec findCorruption bs stack =
        match bs with
        | [] -> None
        | b :: tail ->
            match b.mode with
            | Close ->
                let last = List.head stack

                if last.mode = Close then
                    b.style
                elif last.style = b.style then
                    findCorruption tail (List.tail stack)
                else
                    b.style

            | Open -> findCorruption tail (b :: stack)

    findCorruption brackets []

let part1 =
    inputLines
    |> Seq.map checkLine
    |> Seq.map score
    |> Seq.sum

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
