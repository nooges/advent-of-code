#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Sensor =
    { Pos: Point
      Beacon: Point
      Dist: int }

let input = System.IO.File.ReadAllLines("data/day15-sample.txt")

let dist x1 y1 x2 y2 = abs (x1 - x2) + abs (y1 - y2)
let distToPoint p1 p2 = dist p1.x p1.y p2.x p2.y

let parseLine line =
    let d =
        line
        |> withRegex "x=(.*), y=(.*):.*x=(.*), y=(.*)"
        |> Array.map int

    { Pos = { x = d[0]; y = d[1] }
      Beacon = { x = d[2]; y = d[3] }
      Dist = dist d[0] d[1] d[2] d[3] }

let sensors = input |> Array.map parseLine

let knownBeacons =
    sensors
    |> Array.map (fun s -> s.Beacon)
    |> Set.ofArray

let canHaveBeacon x y =
    let p = { x = x; y = y }

    sensors
    |> Array.forall (fun s -> (distToPoint s.Pos p) > s.Dist)

let beaconsInRow y =
    knownBeacons
    |> Seq.filter (fun b -> b.y = y)
    |> Seq.length

// Find max grid width
let widthRange =
    sensors
    |> Array.map (fun s -> s.Beacon.x)
    |> (fun b -> [ Array.min b; Array.max b ])

//sensors |> pp
widthRange |> pp
//knownBeacons |> pp

let checkY = 10

timeOperation (fun () ->
    //[ 2 * widthRange[0] .. 2 * widthRange[1] ]
    [ 0..4000000 ]
    |> List.filter (fun x -> not (canHaveBeacon x checkY))
    |> List.length
    |> fun c -> c - (beaconsInRow checkY))
|> pp1

sensors
|> Array.iter (fun s -> printfn "(%d, %d) %d" s.Pos.x s.Pos.y s.Dist)

(*
timeOperation (fun () ->
    let grid = Array.init 4000001 (fun _ -> Array.create 4000001 false)
    sensors
    |> Array.iter (fun s -> markSensorCoverage s grid)
) |> pp2
*)

let rowCheck = 11

let sensorRanges y =
    sensors
    |> Array.map (fun s ->
        let dy = abs (y - s.Pos.y)

        [| s.Pos.x - s.Dist + dy
           s.Pos.x + s.Dist - dy |])
    |> Array.filter (fun r -> r[0] < r[1])

let combineRange (a: int []) (b: int []) : int [] [] =
    if a[0] < b[0] then
        if a[1] > b[1] then [| a |] // A overlaps B fully
        elif a[1] < b[0] then [| a; b |] // A and B don't overlap
        else [| [| a[0]; b[1] |] |]
    elif b[1] > a[1] then
        [| b |] // B overlaps A fully
    elif b[1] < a[0] then
        [| a; b |] // A and B don't overlap
    else
        [| [| b[0]; a[1] |] |]

let combineRanges (ranges: int [] []) = true
// let loop n ranges =
//     match ranges with
//     | [|r|] -> ranges
//     | [||]


timeOperation (fun () ->
    [ 0 ]
    |> List.map sensorRanges
    |> List.filter (fun r -> combineRanges r))
// TODO: Combine ranges from 0..4e6
// if combine produces two ranges, stop
|> pp
