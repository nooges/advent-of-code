#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

let input = System.IO.File.ReadAllText("data/day6-input.txt")
//let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"

let findMarker size =
    Seq.windowed size
    >> Seq.findIndex (Set.ofSeq >> Set.count >> (=) size)
    >> (+) size

findMarker 4 input |> pp1
findMarker 14 input |> pp2
