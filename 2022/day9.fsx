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
    { knots: Position []
      pastT: Position Set }

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

let moveKnot (k: Position) (dir: Direction) dist =
    match dir with
    | Up -> { k with y = k.y + dist }
    | Down -> { k with y = k.y - dist }
    | Left -> { k with x = k.x - dist }
    | Right -> { k with x = k.x + dist }

let moveNextKnotOneStep (k: Position) (kn: Position) =
    match (k.x - kn.x, k.y - kn.y) with
    | (dx, dy) when abs dx < 2 && abs dy < 2 -> kn
    | (dx, dy) ->
        { x = kn.x + System.Math.Clamp(dx, -1, 1)
          y = kn.y + System.Math.Clamp(dy, -1, 1) }

let moveKnotsOneStep (knots: Position []) dir =
    let ks' = Array.copy knots
    ks'[0] <- moveKnot knots[0] dir 1

    let rec loop n =
        match n with
        | n when n = knots.Length - 1 -> ks'
        | _ ->
            let kn' = moveNextKnotOneStep ks'[n] ks'[n + 1]
            ks'[n + 1] <- kn'
            loop (n + 1)

    loop 0

let moveRope (state: State) (motion: Motion) =
    let rec loop s step =
        match step with
        | 0 -> s
        | _ ->
            let knots' = moveKnotsOneStep s.knots motion.Direction

            let s' =
                { s with
                    knots = knots'
                    pastT = Set.add (Array.last knots') s.pastT }

            loop s' (step - 1)

    loop state motion.Steps

let motions = input |> Array.map parseLine

let initialState numKnots =
    { knots = Array.create numKnots { x = 0; y = 0 }
      pastT = Set.ofList [ { x = 0; y = 0 } ] }

motions
|> Array.fold moveRope (initialState 2)
|> (fun s -> s.pastT)
|> Set.count
|> pp1

motions
//|> Array.take 2
|> Array.fold moveRope (initialState 10)
|> (fun s -> s.pastT)
|> Set.count
|> pp2
