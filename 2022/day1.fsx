#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day1-input.txt")

let amountCarried =
    inputLines
    |> Seq.map (fun x ->
        match x with
        | "" -> " "
        | _ -> x)
    |> String.concat ","
    |> (fun x -> x.Split " ")
    |> Array.map (fun group -> group.Trim(','))
    |> Array.map (fun group -> group.Split "," |> Array.map int |> Array.sum)

let part1 = amountCarried |> Array.max

let part2 =
    amountCarried
    |> Array.sortDescending
    |> Array.take 3
    |> Array.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
