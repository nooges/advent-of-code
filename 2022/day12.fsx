#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day12-input.txt")

let map =
    input
    |> Array.map (replace "S" "a")
    |> Array.map (replace "E" "z")
    |> Array.map (asCharArray >> Array.map charToInt)

let findLetters letter =
    input
    |> Array.mapi (fun row s ->
        s.ToCharArray()
        |> Array.mapi (fun col c ->
            match c = letter with
            | true -> Some { x = col; y = row }
            | _ -> None))
    |> Array.collect (Array.choose id)

let endPoint = (findLetters 'E')[0]
let numRows = map.Length
let numCols = map[0].Length

let heightDiff a b = map.[b.y].[b.x] - map.[a.y].[a.x]

let possibleMoves p =
    [ { x = p.x + 1; y = p.y }
      { x = p.x - 1; y = p.y }
      { x = p.x; y = p.y + 1 }
      { x = p.x; y = p.y - 1 } ]
    |> List.filter (fun p' ->
        p'.x >= 0
        && p'.x < numCols
        && p'.y >= 0
        && p'.y < numRows
        && heightDiff p' p <= 1)

let fewestSteps endPoint startPoints =
    let costs = Array2D.create numRows numCols 10000
    costs[endPoint.y, endPoint.x] <- 0

    let rec traverse p =
        let cost = costs[p.y, p.x]

        possibleMoves p
        |> List.iter (fun m ->
            match costs[m.y, m.x] > (cost + 1) with
            | true ->
                costs[m.y, m.x] <- cost + 1
                traverse m
            | _ -> ())

    traverse endPoint

    startPoints
    |> Array.map (fun p -> costs[p.y, p.x])
    |> Array.min

fewestSteps endPoint (findLetters 'S') |> pp1
fewestSteps endPoint (findLetters 'a') |> pp2
