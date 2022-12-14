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

let maxDepth = 200

let drawRocks =
    //let grid = Array2D.create 800 200 ' '
    let grid = Array.init (maxDepth + 1) (fun _ -> Array.create 800 ' ')

    paths
    |> Array.iter (fun x -> Array.pairwise x |> Array.iter (drawLine grid))

    grid

let moveGrain (grid: char [] []) x y =
    (x, y) |> pp

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
        | Some (a, b) when b = maxDepth -> (a, b)
        | Some (a, b) -> loop a b

    loop 500 0

let dropSand (grid: char [] []) =
    let rec loop n =
        n |> pp

        match dropGrain grid with
        | (_, y) when y = maxDepth -> n
        | _ -> loop (n + 1)

    loop 0

// Run dropGrain until grid stays the same

paths |> pp

drawRocks
|> Array.map (System.String)
//|> Array.skip 490
|> pp

dropSand drawRocks |> pp

//drawRocks |> Array.map System.String |> pp
