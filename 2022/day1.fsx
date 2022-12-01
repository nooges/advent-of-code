#!/usr/bin/env -S dotnet fsi

open System.IO

let amountCarried =
    File.ReadAllText("data/day1-input.txt")
    |> (fun x -> x.Trim())
    |> (fun x -> x.Split("\n\n"))
    |> Array.map (fun x -> x.Split("\n"))
    |> Array.map (fun group -> group |> Array.map int |> Array.sum)

let part1 = amountCarried |> Array.max

let part2 =
    amountCarried
    |> Array.sortDescending
    |> Array.take 3
    |> Array.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
