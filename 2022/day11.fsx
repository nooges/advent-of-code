#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Monkey =
    { Items: bigint list
      Operation: bigint -> bigint
      Divisor: bigint
      NextTrue: int
      NextFalse: int
      Inspected: bigint }

let input = System.IO.File.ReadAllText("data/day11-input.txt")

let parseItems =
    withRegex "  Starting items: (.*)"
    >> Array.last
    >> split ','
    >> Array.map (trim >> int >> bigint)
    >> Array.toList

let parseOperation line =
    let op =
        line
        |> withRegex "  Operation: new = old (.*)"
        |> Array.last
        |> split ' '

    match op with
    | [| "*"; "old" |] -> (fun x -> x * x)
    | [| "*"; n |] -> (*) (bigint (int n))
    | [| "+"; n |] -> (+) (bigint (int n))
    | _ -> failwith "Unknown"

let parseMonkey (m: string) =
    let lastToInt = split ' ' >> Array.last >> int
    let lines = m.Split '\n'

    { Items = parseItems lines[1]
      Operation = parseOperation lines[2]
      Divisor = lines[3] |> lastToInt |> bigint
      NextTrue = lines[4] |> lastToInt
      NextFalse = lines[5] |> lastToInt
      Inspected = 0I }

let inspectItem op level = (op level) / 3

let primeDivisor = 2I * 3I * 5I * 7I * 11I * 13I * 17I * 19I * 23I
let inspectItem2 (op: bigint -> bigint) (level: bigint) = (op level) % primeDivisor

let addMonkeyItems n newItems (state: Monkey []) =
    { state[n] with Items = List.append state[n].Items newItems }

let processMonkey n (state: Monkey []) =
    let m = state[n]
    let newState = Array.copy state

    m.Items
    |> List.map (inspectItem2 m.Operation)
    |> List.map (fun level ->
        match level % m.Divisor = 0I with
        | true -> (level, m.NextTrue)
        | _ -> (level, m.NextFalse))
    |> List.groupBy (fun (l, next) -> next)
    |> List.map (fun group -> (fst group, snd group |> List.map fst))
    |> List.iter (fun group ->
        let next = fst group
        newState[next] <- addMonkeyItems next (snd group) state)

    newState[n] <- { m with
        Inspected = m.Inspected + bigint m.Items.Length
        Items = [] }

    newState

let processRound (state: Monkey []) =
    let rec loop acc n =
        match n with
        | n when n = state.Length -> acc
        | _ -> loop (processMonkey n acc) (n + 1)

    loop state 0

let runRounds numRounds (state: Monkey []) =
    let rec loop acc n =
        match n with
        | n when n = numRounds -> acc
        | _ -> loop (processRound acc) (n + 1)

    loop state 0

let initialState =
    input
    |> (fun x -> x.Split "\n\n")
    |> Array.map parseMonkey

//[ 0..2 ] |> List.scan processRound initialState

runRounds 20 initialState
|> Array.map (fun m -> m.Inspected)
|> Array.sortDescending
|> Array.take 2
|> Array.reduce (*)
|> pp1


runRounds 10000 initialState
|> Array.map (fun m -> m.Inspected)
|> Array.sortDescending
|> Array.take 2
|> Array.reduce (*)
|> pp2
