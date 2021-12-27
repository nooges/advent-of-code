#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLine = File.ReadLines("data/day7-input.txt")

let fuelCost positions x =
    positions
    |> Array.map (fun p -> abs (p - x))
    |> Array.sum

let fuelCost2 positions x =
    positions
    |> Array.map (fun p -> abs (p - x))
    |> Array.map (fun p -> p * (p + 1) / 2)
    |> Array.sum

let part1 =
    let positions =
        inputLine
        |> Seq.head
        |> (fun x -> x.Split ',')
        |> Array.map int

    let numPos = Array.max positions

    seq { 0..numPos }
    |> Seq.map (fun n -> fuelCost positions n)
    |> Seq.min

let part2 =
    let positions =
        inputLine
        |> Seq.head
        |> (fun x -> x.Split ',')
        |> Array.map int

    let numPos = Array.max positions

    seq { 0..numPos }
    |> Seq.map (fun n -> fuelCost2 positions n)
    |> Seq.min


printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
