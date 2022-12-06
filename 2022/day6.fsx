#!/usr/bin/env -S dotnet fsi

let input = System.IO.File.ReadAllText("data/day6-input.txt")
//let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"

let findMarker size =
    Seq.windowed size
    >> Seq.findIndex (Set.ofSeq >> Set.count >> (=) size)
    >> (+) size

let part1 = findMarker 4 input

let part2 = findMarker 14 input

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
