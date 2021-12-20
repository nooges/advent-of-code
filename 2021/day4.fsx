#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day4-sample.txt")

let draws =
    lines
    |> Seq.head
    |> fun a -> a.Split ','
    |> Array.map int

let boardLinesToArray (boardLines: string []) =
    boardLines
    |> Array.map (fun l ->
        l.Split ' '
        |> Array.filter (fun s -> s <> "")
        |> Array.map int)

let boards =
    lines
    |> Seq.tail
    |> Seq.filter (fun a -> a.Length <> 0)
    |> Seq.chunkBySize 5
    |> Seq.map boardLinesToArray

let markBoard draw board boardMarks =
    (board, boardMarks)
    ||> Array.map2 (fun line lineMarks ->
        (line, lineMarks)
        ||> Array.map2 (fun n mark -> mark || n = draw))

let part1 =
    let initialBoardMarks = Array.create 5 (Array.create 5 false)

    boards
    |> Seq.map (fun b -> markBoard 7 b initialBoardMarks)

let part2 = 0

printfn "Draws: %A" draws
printfn "Boards: %A" boards
printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
