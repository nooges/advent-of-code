#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day5-input.txt")

let dataGroups = input |> (fun x -> x.Split("\n\n"))
let seeds = dataGroups[0] |> allInt64s

type categoryMapRange = { dst: int64; src: int64; len: int64 }

type categoryMaps = categoryMapRange[]

let parseCategoryMap =
    split '\n'
    >> Array.tail
    >> Array.map (allInt64s >> fun a -> { dst = a[0]; src = a[1]; len = a[2] })

let maps = dataGroups |> Array.tail |> Array.map parseCategoryMap

let mapToNewCategory n =
    Array.tryPick (fun m ->
        match n >= m.src && n < (m.src + m.len) with
        | true -> Some(n + m.dst - m.src)
        | _ -> None)
    >> Option.defaultValue n

seeds
|> Array.map (fun s -> maps |> Array.fold mapToNewCategory s)
|> Array.min
|> pp1

// Part 2
type seedRange = { start: int64; len: int64 }

let seedRanges =
    seeds
    |> Array.chunkBySize 2
    |> Array.map (fun a -> { start = a[0]; len = a[1] })

let inSeedRange n =
    seedRanges |> Array.exists (fun m -> n >= m.start && n < (m.start + m.len))

let reversedMaps =
    maps
    |> Array.map (Array.map (fun m -> { m with dst = m.src; src = m.dst }))
    |> Array.rev

seedRanges |> pp

Seq.initInfinite (fun n -> n) // Locations
|> Seq.mapi (fun i s -> (i, reversedMaps |> Array.fold mapToNewCategory s))
|> Seq.filter (fun (loc, s) -> inSeedRange s)
|> Seq.head
|> fst
|> pp2
