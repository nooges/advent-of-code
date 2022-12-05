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

let moveCrates9000 (move: Move) (stacks: char list array) =

    let moveOneCrate (stacks: char list array) =
        let newStacks = Array.copy stacks
        let crate = newStacks[move.Src - 1].Head
        newStacks[move.Dst - 1] <- crate :: stacks[move.Dst - 1]
        newStacks[move.Src - 1] <- stacks[move.Src - 1].Tail
        newStacks

    let rec loop n s =
        match n with
        | 0 -> s
        | _ -> loop (n - 1) (moveOneCrate s)

    loop move.Amount stacks

let moveCrates9001 (move: Move) (stacks: char list array) =
    let newStacks = Array.copy stacks
    let crates = List.take move.Amount stacks[move.Src - 1]
    newStacks[move.Dst - 1] <- List.append crates stacks[move.Dst - 1]
    newStacks[move.Src - 1] <- List.skip move.Amount stacks[move.Src - 1]
    newStacks

let part1 =
    Array.fold (fun move s -> moveCrates9000 s move) initialStacks moves
    |> Array.map List.head
    |> (fun x -> System.String.Concat(x))

let part2 =
    Array.fold (fun move s -> moveCrates9001 s move) initialStacks moves
    |> Array.map List.head
    |> (fun x -> System.String.Concat(x))

printfn "initialStacks: %A" initialStacks
printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
