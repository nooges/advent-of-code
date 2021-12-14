#!/usr/bin/env -S dotnet fsi

open System.IO

let lines = File.ReadLines("data/day2-input.txt")

type Command = {
    direction: string
    distance: int
}

type CourseChange = {
    horizontalChange: int
    depthChange: int
}

let getCommands = 
    lines
    |> Seq.map (fun line -> line.Split ' ')
    |> Seq.map (fun a -> { direction = a[0]; distance = int a[1] } )

let parseCommand command =
    match command.direction with
    | "forward" -> { horizontalChange = command.distance; depthChange = 0 }
    | "down" -> { horizontalChange = 0; depthChange = command.distance }
    | "up" -> { horizontalChange = 0; depthChange = -command.distance }
    | _ -> { horizontalChange = 0; depthChange = 0 }

let part1 =
    let changes = getCommands |> Seq.map parseCommand
    let totalHorizontalChange = changes |> Seq.sumBy (fun c -> c.horizontalChange)
    let totalDepthChange = changes |> Seq.sumBy (fun c -> c.depthChange)
    totalHorizontalChange * totalDepthChange

let part2 = 0

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
