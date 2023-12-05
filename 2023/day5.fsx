#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day5-input.txt")

let dataGroups = input |> (fun x -> x.Split("\n\n"))
let seeds = dataGroups[0] |> allInt64s

type categoryMapRange = { dst: int64; src: int64; len: int64 }

let parseCategoryMap =
    split '\n'
    >> Array.tail
    >> Array.map (allInt64s >> fun a -> { dst = a[0]; src = a[1]; len = a[2] })
    >> Array.sortBy _.src

let maps = dataGroups |> Array.tail |> Array.map parseCategoryMap

let mapToNewCategory n ms =
    let ml = Array.last ms

    match n < ms[0].src || n > (ml.src + ml.len + 1L) with
    | true -> n
    | _ ->
        ms
        |> Array.tryPick (fun m ->
            match n >= m.src && n < (m.src + m.len) with
            | true -> Some(n + m.dst - m.src)
            | _ -> None)
        |> Option.defaultValue n

timeOperation (fun () -> seeds |> Array.map (fun s -> maps |> Array.fold mapToNewCategory s) |> Array.min)
|> pp1

// Part 2
type seedRange = { start: int64; len: int64 }

let seedRanges =
    seeds
    |> Array.chunkBySize 2
    |> Array.map (fun a -> { start = a[0]; len = a[1] })
    |> Array.sortBy _.start

let minSeed = seedRanges[0].start
let maxSeed = (Array.last seedRanges).start + (Array.last seedRanges).len

let inSeedRange n =
    n >= minSeed
    && n < maxSeed
    && seedRanges |> Array.exists (fun m -> n >= m.start && n < (m.start + m.len))

let reversedMaps =
    maps
    |> Array.map (Array.map (fun m -> { m with dst = m.src; src = m.dst }))
    |> Array.rev
    |> Array.map (Array.sortBy _.src)

timeOperation (fun () ->
    let findLocation start step =
        Seq.initInfinite (fun n -> start + int64 n * step) // Locations
        |> Seq.mapi (fun i s -> (int64 i + start, reversedMaps |> Array.fold mapToNewCategory s))
        |> Seq.filter (snd >> inSeedRange)
        |> Seq.head
        |> fst

    // There's probably a set of inputs possible that would make this estimation trick not work,
    // like having a range getting chopped up into pieces smaller than the step size
    let stepSize = 10000L
    let estimate = findLocation 0L stepSize
    estimate * stepSize |> pp
    findLocation ((estimate - 1L) * stepSize) 1L)
|> pp2
