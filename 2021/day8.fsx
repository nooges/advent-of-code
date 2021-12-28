#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day8-input.txt")

type Note =
    { signals: string []
      outputs: string [] }

type Note2 =
    { signalSets: seq<Set<char>>
      outputSets: seq<Set<char>> }

let notes =
    inputLines
    |> Seq.map (fun l -> l.Split " | ")
    |> Seq.map (fun l ->
        { signals = l.[0].Split " "
          outputs = l.[1].Split " " })

let decodeNote (note: Note2) =

    // Find sets corresponding to 1, 4, 7, and 8
    let set1 = note.signalSets |> Seq.find (fun s -> s.Count = 2)
    let set4 = note.signalSets |> Seq.find (fun s -> s.Count = 4)
    let set7 = note.signalSets |> Seq.find (fun s -> s.Count = 3)
    let set8 = note.signalSets |> Seq.find (fun s -> s.Count = 7)

    // Find sets corresponding to 0, 6, and 9 (6 segments)
    let seg6sets =
        note.signalSets
        |> Seq.filter (fun s -> s.Count = 6)

    let set9 =
        seg6sets
        |> Seq.find (fun s -> s.IsSupersetOf set4)

    let set6 =
        seg6sets
        |> Seq.find ((fun s -> s.IsSupersetOf set1) >> not)

    let set0 =
        seg6sets
        |> Seq.find (fun s -> s <> set9 && s <> set6)

    // Find sets corresponding to 2, 3, and 5 (5 segments)
    let seg5sets =
        note.signalSets
        |> Seq.filter (fun s -> s.Count = 5)

    let set3 =
        seg5sets
        |> Seq.find (fun s -> s.IsSupersetOf set1)

    let set5 = seg5sets |> Seq.find (fun s -> s.IsSubsetOf set6)

    let set2 =
        seg5sets
        |> Seq.find (fun s -> s <> set3 && s <> set5)

    let decoderMap =
        Map
            .empty
            .Add(set1, 1)
            .Add(set2, 2)
            .Add(set3, 3)
            .Add(set4, 4)
            .Add(set5, 5)
            .Add(set6, 6)
            .Add(set7, 7)
            .Add(set8, 8)
            .Add(set9, 9)
            .Add(set0, 0)

    note.outputSets
    |> Seq.map (fun s -> decoderMap.[s])
    |> Seq.reduce (fun acc d -> acc * 10 + d)

let part1 =
    printfn "%A" notes

    notes
    |> Seq.map (fun note ->
        note.outputs
        |> Array.map (fun a -> a.Length)
        |> Array.filter (fun a -> Array.contains a [| 2; 3; 4; 7 |])
        |> Array.length)
    |> Seq.sum

let part2 =

    let notes2 =
        inputLines
        |> Seq.map (fun l -> l.Split " | ")
        |> Seq.map (fun l ->
            { signalSets =
                l.[0].Split " "
                |> Seq.map (fun s -> s |> Seq.toList |> Set.ofList)
              outputSets =
                l.[1].Split " "
                |> Seq.map (fun s -> s |> Seq.toList |> Set.ofList) })

    printfn "%A" notes2

    notes2
    |> Seq.map decodeNote
    |> Seq.toList
    |> Seq.sum

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
