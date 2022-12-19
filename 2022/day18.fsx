#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day18-input.txt")

let cubes = input |> Array.map (split ',' >> Array.map int) |> Set.ofArray

let add (a: int[]) (b: int[]) = (a, b) ||> Array.map2 (+)

let lookDirections =
    [ [| 0; 0; 1 |]
      [| 0; 0; -1 |]
      [| 0; 1; 0 |]
      [| 0; -1; 0 |]
      [| 1; 0; 0 |]
      [| -1; 0; 0 |] ]

let exposedFaces (a: int[]) =
    lookDirections
    |> List.filter (fun c -> not (cubes.Contains(add a c)))
    |> List.length

let touchedFaces (a: int[]) =
    lookDirections |> List.filter (fun c -> cubes.Contains(add a c)) |> List.length

cubes |> Seq.map exposedFaces |> Seq.sum |> pp1

let maxCoords =
    [ 0..2 ]
    |> List.map (fun dim ->
        let dimVals = cubes |> Seq.map (fun c -> c[dim])
        [| Seq.min dimVals; Seq.max dimVals |])

maxCoords |> pp

// Traverse in range of bounding cube to find every exterior spot without a cube,
// then go through that list and sum up all the faces touched

let minDim = -2
let maxDim = 22
let mutable visited = set [||]

// If c has no cubes around it, add to set
let rec traverse (c: int[]) =
    visited <- visited.Add c

    lookDirections
    |> List.iter (fun d ->
        let c' = add c d

        if
            c'[0] > minDim
            && c'[0] < maxDim
            && c'[1] > minDim
            && c'[1] < maxDim
            && c'[2] > minDim
            && c'[2] < maxDim
            && not (cubes.Contains c')
            && not (visited.Contains c')
        then
            traverse c'
        else
            ())

traverse [| 0; 0; 0 |]

visited |> Seq.map touchedFaces |> Seq.sum |> pp2
