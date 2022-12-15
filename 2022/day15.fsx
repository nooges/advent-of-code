#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Sensor =
    { Pos: Point
      Beacon: Point
      Dist: int }

let input = System.IO.File.ReadAllLines("data/day15-input.txt")

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

timeOperation (fun () ->
    [ 2 * widthRange[0] .. 2 * widthRange[1] ]
    |> List.filter (fun x -> not (canHaveBeacon x 2000000))
    |> List.length
    |> fun c -> c - (beaconsInRow 2000000))
|> pp1

(beaconsInRow 2000000) |> pp
