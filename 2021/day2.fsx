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

type ChangeType = 
    | Horizontal
    | Aim
    | None
type CourseChange2 = {
    changeType: ChangeType
    value: int
}

type CourseState = {
    horizontalPosition: int
    depth: int
    aim: int
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

let parseCommand2 command =
    match command.direction with
    | "forward" -> { changeType = Horizontal; value = command.distance }
    | "down" -> { changeType = Aim; value = command.distance }
    | "up" -> { changeType = Aim; value = -command.distance }
    | _ -> { changeType = None; value = 0 }

let processCommand state command =
    match command.changeType with
    | Horizontal -> { state with horizontalPosition = state.horizontalPosition + command.value; depth = state.depth + state.aim * command.value }
    | Aim -> { state with aim = state.aim + command.value}
    | _ -> state

let part2 = 
    let changes = getCommands |> Seq.map parseCommand2
    let initialState = { horizontalPosition = 0; depth = 0; aim = 0 }
    let totalChange =
        changes
        |> Seq.fold processCommand initialState
    totalChange.horizontalPosition * totalChange.depth

printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
