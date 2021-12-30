#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLines = File.ReadLines("data/day9-input.txt")

let adjacentPoints x y map =
    let maxX = map |> Array2D.length1
    let maxY = map |> Array2D.length2

    let points =
        [| (x - 1, y)
           (x + 1, y)
           (x, y - 1)
           (x, y + 1) |]

    points
    |> Array.filter (fun (x2, y2) -> x2 >= 0 && x2 < maxX && y2 >= 0 && y2 < maxY)

let isLowPoint height adjPoints (map: int [,]) =
    adjPoints
    |> Array.forall (fun (x, y) -> map.[x, y] > height)

let riskLevel height adjPoints (map: int [,]) =
    adjPoints
    |> Array.forall (fun (x, y) -> map.[x, y] > height)
    |> (fun x ->
        match x with
        | true -> height + 1
        | false -> 0)

let unmarkedNeighbors x y (marks: Option<int> [,]) =
    adjacentPoints x y marks
    |> Array.filter (fun (x2, y2) -> marks.[x2, y2] = None)

let hmap =
    inputLines
    |> Seq.map (fun line ->
        line
        |> Seq.toList
        |> List.map string
        |> Seq.map int)
    |> array2D

let part1 =
    hmap
    |> Array2D.mapi (fun x y height -> riskLevel height (adjacentPoints x y hmap) hmap)
    |> Seq.cast<int>
    |> Seq.sum

let part2 =
    // Generate basin marks array
    let basinMarks =
        hmap
        |> Array2D.map (fun h ->
            match h with
            | 9 -> Some -1
            | _ -> None)

    // Mark low points
    let lowPoints =
        hmap
        |> Array2D.mapi (fun x y h ->
            match (isLowPoint h (adjacentPoints x y hmap) hmap) with
            | true -> Some(x, y)
            | false -> None)
        |> Seq.cast<Option<int * int>>
        |> Seq.choose id

    lowPoints
    |> Seq.iteri (fun i (x, y) -> basinMarks.[x, y] <- Some i)

    let rec findBasinPoints x y h marks : Set<(int * int)> =
        let near = unmarkedNeighbors x y marks

        match near.Length with
        | 0 -> Set.empty
        | _ ->
            near
            |> Array.iter (fun (xi, yi) -> marks.[xi, yi] <- Some h)

            let newPoints =
                near
                |> Array.map (fun (xi, yi) -> findBasinPoints xi yi h marks)
                |> Set.unionMany

            Set.union (near |> Set.ofArray) newPoints

    // For each low point, find unmarked areas around them and mark
    lowPoints
    |> Seq.mapi (fun i (x, y) -> findBasinPoints x y i basinMarks)
    |> Seq.map (fun b -> b.Count + 1)
    |> Seq.sortDescending
    |> Seq.take 3
    |> Seq.reduce (*)

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
