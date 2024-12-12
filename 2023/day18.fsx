#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
#load "Array2D-ext.fs"

open AOC.Utils
open AOC.Array2D

let input = System.IO.File.ReadAllLines("data/day18-sample.txt")

type Instruction =
    { dir: string
      dist: int
      color: string }

let plan =
    input
    |> Array.map (fun l ->
        l
        |> withRegex "(.*) (.*) \(#(.*)\)"
        |> (fun a ->
            { dir = a[0]
              dist = int a[1]
              color = a[2] }))

//plan |> pp

let rec traverse pos l =
    