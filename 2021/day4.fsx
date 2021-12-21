#!/usr/bin/env -S dotnet fsi

open System.IO
type WinningInfo = { lastDraw: int; marks: bool [] [] [] }

let lines = File.ReadLines("data/day4-input.txt")

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
    |> Seq.toArray

let markBoard draw board boardMarks =
    (board, boardMarks)
    ||> Array.map2 (fun line lineMarks ->
        (line, lineMarks)
        ||> Array.map2 (fun n mark -> mark || n = draw))

let isBoardComplete boardMarks =
    // Check rows
    let hasWinningRow =
        boardMarks
        |> Array.map (fun line -> line |> Array.forall id)
        |> Array.contains true

    let hasWinningCol =
        boardMarks
        |> Array.reduce (fun state line -> (state, line) ||> Array.map2 (fun x y -> x && y))
        |> Array.contains true

    hasWinningRow || hasWinningCol

let part1 =
    let initialBoardMarks =
        Array.create (Seq.length boards) (Array.create 5 (Array.create 5 false))

    let winningInfo =
        let rec markedBoardsHelper lastDraw (draws: int []) boards boardMarks =
            let hasWinningBoard =
                boardMarks
                |> Array.map isBoardComplete
                |> Array.contains true

            match hasWinningBoard with
            | true ->
                { marks = boardMarks
                  lastDraw = lastDraw }
            | false ->
                markedBoardsHelper
                    draws.[0]
                    (Array.tail draws)
                    boards
                    ((boards, boardMarks)
                     ||> Array.map2 (fun b m -> markBoard draws.[0] b m))

        markedBoardsHelper 0 draws boards initialBoardMarks

    let winningBoardNum =
        winningInfo.marks
        |> Array.map isBoardComplete
        |> Array.findIndex id

    let unmarkedSum =
        (boards.[winningBoardNum], winningInfo.marks.[winningBoardNum])
        ||> Array.map2 (fun b m ->
            (b, m)
            ||> Array.map2 (fun e1 e2 -> if e2 then 0 else e1)
            |> Array.sum)
        |> Array.sum

    unmarkedSum * winningInfo.lastDraw

//boards
//|> Seq.map (fun b -> markBoard 7 b initialBoardMarks)

let part2 = 0

printfn "Draws: %A" draws
printfn "Boards: %A" boards
printfn "Part 1: %A" part1
printfn "Part 2: %A" part2

let testColWin =
    [| [| false; false; false; false; true |]
       [| false; false; false; false; true |]
       [| false; false; false; false; true |]
       [| false; false; false; false; true |]
       [| false; false; false; false; true |] |]

let testRowWin =
    [| [| false; false; false; false; true |]
       [| false; false; false; false; true |]
       [| true; true; true; true; true |]
       [| false; false; false; false; true |]
       [| false; false; false; false; false |] |]

//printfn "Complete: %A" (isBoardComplete testColWin)
//printfn "Complete: %A" (isBoardComplete testRowWin)
