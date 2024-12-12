#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
#load "Array2D-ext.fs"

open AOC.Utils
open AOC.Array2D

let input = System.IO.File.ReadAllLines("data/day17-sample.txt")

let grid = input |> array2D |> Array2D.map charDigitToInt

grid |> maxC |> pp
