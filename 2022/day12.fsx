#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day12-input.txt")

let heightMap =
    input
    |> Array.map (replace "S" "a")
    |> Array.map (replace "E" "z")
    |> Array.map (asCharArray >> Array.map charToInt)

let findLetter c =
    let row =
        input
        |> Array.map (contains c)
        |> Array.findIndex id

    let col =
        input[ row ].ToCharArray()
        |> Array.findIndex ((=) c)

    { x = col; y = row }

let startPoint = findLetter 'S'
let endPoint = findLetter 'E'
let numRows = heightMap.Length
let numCols = heightMap[0].Length

let visited = Array2D.create numRows numCols 10000
visited[startPoint.y, startPoint.x] <- 0

heightMap |> pp

let heightDiff a b =
    heightMap.[b.y].[b.x] - heightMap.[a.y].[a.x]

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
        && heightDiff p p' <= 1)

let rec traverse p =
    let cost = visited[p.y, p.x]

    possibleMoves p
    |> List.iter (fun m ->
        match visited[m.y, m.x] > (cost + 1) with
        | true ->
            visited[m.y, m.x] <- cost + 1
            traverse m
        | _ -> ())

traverse startPoint

visited |> pp

visited[endPoint.y, endPoint.x] |> pp1
