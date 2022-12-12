#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Monkey =
    { Items: int64 list
      Operation: int64 -> int64
      Divisor: int64
      NextTrue: int
      NextFalse: int
      Inspected: int }

let input = System.IO.File.ReadAllText("data/day11-input.txt")

let parseItems =
    withRegex "  Starting items: (.*)"
    >> Array.last
    >> split ','
    >> Array.map (trim >> int64)
    >> Array.toList

let parseOperation line =
    let op =
        line
        |> withRegex "  Operation: new = old (.*)"
        |> Array.last
        |> split ' '

    match op with
    | [| "*"; "old" |] -> (fun x -> x * x)
    | [| "*"; n |] -> (*) (int64 n)
    | [| "+"; n |] -> (+) (int64 n)
    | _ -> failwith "Unknown"

let parseMonkey (m: string) =
    let lastToInt = split ' ' >> Array.last >> int
    let lines = m.Split '\n'

    { Items = parseItems lines[1]
      Operation = parseOperation lines[2]
      Divisor = lines[3] |> lastToInt |> int64
      NextTrue = lines[4] |> lastToInt
      NextFalse = lines[5] |> lastToInt
      Inspected = 0 }

let monkeys =
    input
    |> (fun x -> x.Split "\n\n")
    |> Array.map parseMonkey

let modulus =
    monkeys
    |> Array.map (fun m -> m.Divisor)
    |> Array.reduce (*)

let inspectItem op level = (op level) / 3

let inspectItem2 op level = (op level) % modulus

let processMonkey (items: int64 list array) n =
    items[n]
    |> List.map (inspectItem2 monkeys[n].Operation)
    |> List.iter (fun level ->
        let next =
            match level % monkeys[n].Divisor = 0L with
            | true -> monkeys[n].NextTrue
            | _ -> monkeys[n].NextFalse

        items[next] <- items[next] @ [ level ])

    monkeys[n] <- { monkeys[n] with Inspected = monkeys[n].Inspected + items[n].Length }
    items[n] <- []
    items

let processRound (items: int64 list array) =
    (items, [ 0 .. monkeys.Length - 1 ])
    ||> List.fold processMonkey

let runRounds numRounds (items: int64 list array) =
    let rec loop acc n =
        match n with
        | n when n = numRounds -> acc
        | _ -> loop (processRound acc) (n + 1)

    loop items 0

let initialItems = monkeys |> Array.map (fun m -> m.Items)

(*
runRounds 20 initialItems
|> Array.map (fun m -> m.Inspected)
|> Array.sortDescending
|> Array.take 2
|> Array.reduce (*)
|> pp1
*)


runRounds 10000 initialItems

monkeys
|> Array.map (fun m -> int64 m.Inspected)
|> Array.sortDescending
|> Array.take 2
|> Array.reduce (*)
|> pp2
