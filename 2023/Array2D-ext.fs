namespace AOC

open System
open System.Collections.Generic

module Array2D =
    type GridPoint = { r: int; c: int }

    type Dir =
        | Up
        | Down
        | Left
        | Right

    let maxR grid = Array2D.length1 grid - 1
    let maxC grid = Array2D.length2 grid - 1
    let rowRange grid = [ 0 .. maxR grid ]
    let colRange grid = [ 0 .. maxC grid ]

    let move (p: GridPoint) dir =
        match dir with
        | Dir.Up -> (dir, { p with r = p.r - 1 })
        | Dir.Down -> (dir, { p with r = p.r + 1 })
        | Dir.Left -> (dir, { p with c = p.c - 1 })
        | Dir.Right -> (dir, { p with c = p.c + 1 })
