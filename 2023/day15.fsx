#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day15-input.txt")

let hash = asCharArray >> Array.fold (fun acc c -> ((acc + int c) * 17) % 256) 0

// Part 1
timeOperation (fun () -> input |> split ',' |> Array.sumBy hash) |> pp1

// Part 2
type Step =
    { label: string
      box: int
      power: int option }

type Lens = { label: string; power: int }

let parseStep (s: string) =

    match s.EndsWith('-') with
    | true ->
        let label = s[.. s.Length - 2]

        { label = label
          box = hash label
          power = None }
    | _ ->
        let label = s[.. s.Length - 3]

        { label = label
          box = hash label
          power = Some(s |> Seq.last |> charDigitToInt) }

let processStep (boxes: Lens list[]) step =
    let lensIndex = boxes[step.box] |> List.tryFindIndex (fun l -> l.label = step.label)

    match step.power with
    | None ->
        match lensIndex with
        | None -> ()
        | Some i -> boxes[step.box] <- boxes[step.box] |> List.removeAt i
    | Some power ->
        let lens = { label = step.label; power = power }

        match lensIndex with
        | None -> boxes[step.box] <- lens :: boxes[step.box]
        | Some i -> boxes[step.box] <- boxes[step.box] |> List.updateAt i lens

    boxes

timeOperation (fun () ->
    input
    |> split ','
    |> Array.map parseStep
    |> Array.fold processStep (Array.init 256 (fun _ -> []))
    |> Array.mapi (fun n box ->
        box
        |> List.rev
        |> List.mapi (fun i lens -> (n + 1) * (i + 1) * lens.power)
        |> List.sum)
    |> Array.sum)
|> pp2
