#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day24-sample.txt")

(*
   x = (FH−BD)/(AC−EG)
​   y = (AH−CF)/(AC−EG)
​   t1 = (F-B)/(A-E)
   t2 = (H-D)/(C-G)
*)

(*
    1. Check if parallel
*)