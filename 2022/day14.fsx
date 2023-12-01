#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day14-input.txt")

let paths =
    input
    |> Array.map (
        splitStr " -> "
        >> Array.map ((split ',') >> Array.map int)
    )

let drawLine (grid: char [] []) (pair: int [] * int []) =
    match pair with
    | a, b when a[0] = b[0] ->
        let start = min a[1] b[1]
        let stop = max a[1] b[1]

        [ start..stop ]
        |> List.iter (fun n -> grid.[n][a[0]] <- '█')
    | a, b when a[1] = b[1] ->
        let start = min a[0] b[0]
        let stop = max a[0] b[0]

        [ start..stop ]
        |> List.iter (fun n -> grid.[a[1]][n] <- '█')
    | _ -> ()

let maxGridDepth = 200
let maxGridWidth = 800

let drawRocks =
    let grid = Array.init (maxGridDepth + 1) (fun _ -> Array.create maxGridWidth ' ')

    paths
    |> Array.iter (fun x -> Array.pairwise x |> Array.iter (drawLine grid))

    grid

let maxDepth =
    drawRocks
    |> Array.map (System.String)
    |> Array.findIndexBack (contains '█')
    |> (+) 2

let moveGrain (grid: char [] []) x y =
    if grid.[y + 1][x] = ' ' then
        Some(x, y + 1)
    elif grid.[y + 1][x - 1] = ' ' then
        Some(x - 1, y + 1)
    elif grid.[y + 1][x + 1] = ' ' then
        Some(x + 1, y + 1)
    else
        None

let dropGrain (grid: char [] []) =
    let rec loop x y =
        match moveGrain grid x y with
        | None ->
            grid.[y][x] <- 'o'
            (x, y)
        | Some (a, b) when b = maxGridDepth -> (a, b)
        | Some (a, b) -> loop a b

    loop 500 0

let dropSand (grid: char [] []) =
    let rec loop n =
        match dropGrain grid with
        | (_, y) when y = maxGridDepth -> n
        | _ -> loop (n + 1)

    loop 0

let dropMoreSandUntilBlocked (grid: char [] []) =
    grid.[maxDepth] <- Array.create maxGridWidth '█'

    let rec loop n =
        match dropGrain grid with
        | (_, y) when y = 0 -> n + 1
        | _ -> loop (n + 1)

    loop 0

let part1 = dropSand drawRocks
part1 |> pp1

timeOperation (fun () -> dropMoreSandUntilBlocked drawRocks)
|> pp
//let part2 = part1 + dropMoreSandUntilBlocked drawRocks
//part2 |> pp2

(*

drawRocks
|> Array.map System.String
|> (fun s -> System.String.Join("\n", s))
|> pp

*)
