#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Direction =
    | Up
    | Down
    | Left
    | Right

type Motion = { Direction: Direction; Steps: int }

type Position = { x: int; y: int }

type State =
    { h: Position
      t: Position
      pastT: Position List }

let input = System.IO.File.ReadAllLines("data/day9-input.txt")

let parseLine line =
    line
    |> split ' '
    |> (fun a ->
        match a with
        | [| "U"; n |] -> { Direction = Up; Steps = int n }
        | [| "D"; n |] -> { Direction = Down; Steps = int n }
        | [| "L"; n |] -> { Direction = Left; Steps = int n }
        | [| "R"; n |] -> { Direction = Right; Steps = int n }
        | _ -> failwith "Unknown input")

let stepOne (s: State) (dir: Direction) =
    let h' =
        match dir with
        | Up -> { s.h with y = s.h.y + 1 }
        | Down -> { s.h with y = s.h.y - 1 }
        | Left -> { s.h with x = s.h.x - 1 }
        | Right -> { s.h with x = s.h.x + 1 }

    let tailTooFar = abs (h'.x - s.t.x) > 1 || abs (h'.y - s.t.y) > 1

    match tailTooFar with
    | false -> { s with h = h' }
    | _ ->
        let t' =
            match dir with
            | Up -> { h' with y = h'.y - 1 }
            | Down -> { h' with y = h'.y + 1 }
            | Left -> { h' with x = h'.x + 1 }
            | Right -> { h' with x = h'.x - 1 }

        { s with
            h = h'
            t = t'
            pastT = t' :: s.pastT }

let moveRope (state: State) (motion: Motion) =
    let rec loop s step =
        match step with
        | 0 -> s
        | _ -> loop (stepOne s motion.Direction) (step - 1)

    loop state motion.Steps

let motions = input |> Array.map parseLine

let initialState =
    { h = { x = 0; y = 0 }
      t = { x = 0; y = 0 }
      pastT = [ { x = 0; y = 0 } ] }

motions
|> Array.fold moveRope initialState
|> (fun s -> s.pastT)
|> Set.ofList
|> Set.count
|> pp1
