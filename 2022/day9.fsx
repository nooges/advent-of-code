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

let input = System.IO.File.ReadAllLines("data/day9-sample2.txt")

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

let moveKnotPairOneStep (k: Position) (kn: Position) (dir: Direction) isHead =
    let k' =
        match isHead with
        | true -> moveKnot k dir 1
        | _ -> k

    let nextKnotTooFar = abs (k'.x - kn.x) > 1 || abs (k'.y - kn.y) > 1

    // TODO: Figure out correct direction to move
    let d =
        match (k', kn) with
        | (k', kn) when k'.x - kn.x > 1 -> Right
        | (k', kn) when k'.x - kn.x < -1 -> Left
        | (k', kn) when k'.y - kn.y > 1 -> Up
        | _ -> Down

    match nextKnotTooFar with
    | false -> (k', kn)
    | _ -> (k', moveKnot k' d -1)

let moveKnotsOneStep (knots: Position []) dir =
    let ks' = Array.copy knots

    let rec loop n =
        match n with
        | n when n = knots.Length - 1 -> ks'
        | _ ->
            let (k', kn') = moveKnotPairOneStep ks'[n] ks'[n + 1] dir (n = 0)
            ks'[n] <- k'
            ks'[n + 1] <- kn'
            //(n, ks') |> pp
            loop (n + 1)

    loop 0

let moveRope (state: State) (motion: Motion) =
    let rec loop s step =
        match step with
        | 0 -> s
        | _ ->
            let knots' = moveKnotsOneStep s.knots motion.Direction
            //knots' |> pp

            let s' =
                { s with
                    knots = knots'
                    pastT = Set.add (Array.last knots') s.pastT }

            loop s' (step - 1)

    let state' = loop state motion.Steps
    //"***************" |> pp
    //Array.last state'.knots |> pp
    //state'.pastT |> pp
    state'

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
