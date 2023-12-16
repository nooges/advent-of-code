#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllLines("data/day16-input.txt")

let grid = input |> array2D
let maxRow = Array2D.length1 grid - 1
let maxCol = Array2D.length2 grid - 1
let rowRange = [ 0..maxRow ]
let colRange = [ 0..maxCol ]

type Dir =
    | Up
    | Down
    | Left
    | Right

let move (beam: Dir * GridPoint) dir =
    let p = snd beam

    match dir with
    | Dir.Up -> (dir, { p with r = p.r - 1 })
    | Dir.Down -> (dir, { p with r = p.r + 1 })
    | Dir.Left -> (dir, { p with c = p.c - 1 })
    | Dir.Right -> (dir, { p with c = p.c + 1 })

let nextState beam =
    let p = snd beam

    let newBeams =
        match (fst beam, grid[p.r, p.c]) with
        | (dir, '.') -> [ move beam dir ]
        | (Dir.Up, '|') -> [ move beam Dir.Up ]
        | (Dir.Up, '-') -> [ move beam Dir.Left; move beam Dir.Right ]
        | (Dir.Up, '\\') -> [ move beam Dir.Left ]
        | (Dir.Up, '/') -> [ move beam Dir.Right ]
        | (Dir.Down, '|') -> [ move beam Dir.Down ]
        | (Dir.Down, '-') -> [ move beam Dir.Left; move beam Dir.Right ]
        | (Dir.Down, '\\') -> [ move beam Dir.Right ]
        | (Dir.Down, '/') -> [ move beam Dir.Left ]
        | (Dir.Left, '-') -> [ move beam Dir.Left ]
        | (Dir.Left, '|') -> [ move beam Dir.Up; move beam Dir.Down ]
        | (Dir.Left, '\\') -> [ move beam Dir.Up ]
        | (Dir.Left, '/') -> [ move beam Dir.Down ]
        | (Dir.Right, '-') -> [ move beam Dir.Right ]
        | (Dir.Right, '|') -> [ move beam Dir.Up; move beam Dir.Down ]
        | (Dir.Right, '\\') -> [ move beam Dir.Down ]
        | (Dir.Right, '/') -> [ move beam Dir.Up ]
        | _ -> []

    newBeams
    |> List.filter (fun (_, p) -> isBetween 0 maxRow p.r && isBetween 0 maxCol p.c)

let rec traverse (visited: Map<Dir * GridPoint, bool>) beams =
    let beams' =
        beams
        |> List.filter (fun beam -> not (visited.ContainsKey beam))
        |> List.distinct

    match beams' with
    | [] -> visited
    | x :: xs ->
        let visited' = visited |> Map.add x true
        traverse visited' ((nextState x) @ xs)

let countEnergized initState =
    traverse (Map []) [ initState ] |> Map.keys |> Seq.distinctBy snd |> Seq.length

timeOperation (fun () -> countEnergized (Dir.Right, { r = 0; c = 0 })) |> pp1

timeOperation (fun () ->
    (rowRange |> List.map (fun r -> (Dir.Right, { r = r; c = 0 })))
    @ (rowRange |> List.map (fun r -> (Dir.Left, { r = r; c = maxCol })))
    @ (colRange |> List.map (fun c -> (Dir.Down, { r = 0; c = c })))
    @ (colRange |> List.map (fun c -> (Dir.Right, { r = maxRow; c = c })))
    |> List.map countEnergized
    |> List.max)
|> pp2
