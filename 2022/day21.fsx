#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Collections.Generic

type Job =
    | Number of int64
    | Equation of string

type Monkey = { Name: string; Value: Job }

let input = System.IO.File.ReadAllLines("data/day21-sample.txt")

let isAllDigits (s: string) =
    s |> Seq.forall (fun c -> System.Char.IsDigit(c))

let canEvaluate (s: string) =
    s |> Seq.forall (fun c -> not (System.Char.IsLetter(c)))

let parseLine line =
    line
    |> withRegex "(.*): (.*)"
    |> (fun x ->
        match isAllDigits x[1] with
        | true ->
            { Name = x[0]
              Value = Number(int64 x[1]) }
        | _ -> { Name = x[0]; Value = Equation(x[1]) })

let evaluate (s: string) =
    let x = s.Split(' ')

    match x[1] with
    | "-" -> int64 x[0] - int64 x[2]
    | "+" -> int64 x[0] + int64 x[2]
    | "*" -> int64 x[0] * int64 x[2]
    | "/" -> int64 x[0] / int64 x[2]
    | _ -> failwith "Unknown operator"

let substituteValues (monkeyEquations: Dictionary<string, string>) (monkeyValues: Dictionary<string, int64>) =
    monkeyEquations
    |> Seq.iter (fun (KeyValue(name, eq)) ->
        let mutable eq' = eq

        monkeyValues
        |> Seq.iter (fun (KeyValue(k, n)) ->
            if eq.Contains(k) then
                eq' <- eq'.Replace(k, string n))

        monkeyEquations[name] <- eq')

let evaluateJobs (monkeyEquations: Dictionary<string, string>) (monkeyValues: Dictionary<string, int64>) =
    monkeyEquations
    |> Seq.iter (fun (KeyValue(name, eq)) ->
        if canEvaluate eq then
            monkeyValues[name] <- evaluate eq
            let _ = monkeyEquations.Remove name
            ())

let part1 () =

    let monkeyEquations = new Dictionary<string, string>()
    let monkeyValues = new Dictionary<string, int64>()

    input
    |> Array.map parseLine
    |> Array.iter (fun m ->
        match m.Value with
        | Number n -> monkeyValues.Add(m.Name, n)
        | Equation eq -> monkeyEquations.Add(m.Name, eq))

    let rec loop n =
        printfn "%d: %d" n monkeyEquations.Count

        match monkeyEquations.Count with
        | 0 -> monkeyValues["root"]
        //| _ when monkeyValues.ContainsKey "root" -> monkeyValues["root"]
        | _ ->
            substituteValues monkeyEquations monkeyValues
            evaluateJobs monkeyEquations monkeyValues
            loop (n + 1)

    loop 1

timeOperation part1 |> pp1

let part2 () =

    let monkeyEquations = new Dictionary<string, string>()
    let monkeyValues = new Dictionary<string, int64>()

    input
    |> Array.map parseLine
    |> Array.iter (fun m ->
        match m.Value with
        | Number n -> monkeyValues.Add(m.Name, n)
        | Equation eq -> monkeyEquations.Add(m.Name, eq))

    let rec loop n =
        printfn "%d: %d" n monkeyEquations.Count

        match monkeyEquations.Count with
        | 0 -> monkeyValues["root"]
        //| _ when monkeyValues.ContainsKey "root" -> monkeyValues["root"]
        | _ ->
            substituteValues monkeyEquations monkeyValues
            evaluateJobs monkeyEquations monkeyValues
            loop (n + 1)

    loop 1

timeOperation part2 |> pp1
