#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Move = { Amount: int; Src: int; Dst: int }

let inputLines = System.IO.File.ReadAllLines("data/day5-input.txt")

let startLineNum =
    inputLines
    |> Array.findIndex (fun l -> l.StartsWith "move")

let numStacks =
    inputLines[startLineNum - 2]
    |> split ' '
    |> Array.filter (fun x -> not (System.String.IsNullOrWhiteSpace x))
    |> Array.last
    |> int

let initialStacks =
    let stackInput = inputLines |> Array.take (startLineNum - 2)

    let getStack n =
        stackInput
        |> Array.map (fun x -> x[4 * n - 3])
        |> Array.filter (fun c -> c <> ' ')
        |> Array.toList

    [ 1..numStacks ]
    |> List.map getStack
    |> List.toArray

let parseMove (line: string) =
    line
    |> replace "move " ""
    |> replace " from " ","
    |> replace " to " ","
    |> split ','
    |> Array.map int
    |> (fun a ->
        { Amount = a[0]
          Src = a[1]
          Dst = a[2] })

let moves =
    inputLines
    |> Array.skip startLineNum
    |> Array.map parseMove

let moveOneCrate (src: int) (dst: int) (stacks: char list array) =
    let crate = stacks[src - 1].Head
    stacks[dst - 1] <- crate :: stacks[dst - 1]
    stacks[src - 1] <- stacks[src - 1].Tail
    //printfn "new stacks: %A" stacks
    stacks

let moveCrates (move: Move) (stacks: char list array) =
    let rec loop n s =
        match n with
        | 0 -> s
        | _ -> loop (n - 1) (moveOneCrate move.Src move.Dst s)

    loop move.Amount stacks

let part1 =
    Array.fold (fun move s -> moveCrates s move) initialStacks moves
    |> Array.map List.head
    |> (fun x -> System.String.Concat(x))

let part2 = 0

printfn "initialStacks: %A" initialStacks
printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
printfn "startLineNum: %A" startLineNum
printfn "numStacks: %A" numStacks
//printfn "moves: %A" moves
