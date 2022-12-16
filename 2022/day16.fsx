#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

open System.Collections.Generic

type Valve =
    { Label: string
      Opened: bool
      FlowRate: int
      Tunnels: string [] }

type State =
    { Pressure: int
      Rate: int
      Unopened: string Set
      LastRateChange: int
      Path: string list
      Rates: int list }

let input = System.IO.File.ReadAllLines("data/day16-input.txt")

let parseLine line =
    line
    |> withRegex "Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)"
    |> (fun x ->
        { Label = x[0]
          Opened = int x[1] = 0
          FlowRate = int x[1]
          Tunnels = x[2] |> splitStr ", " })

let valves = input |> Array.map parseLine

let valvesDict = new Dictionary<string, Valve>()

valves
|> Array.iter (fun v -> valvesDict.Add(v.Label, v))

let unopenedValves =
    valves
    |> Array.filter (fun v -> not v.Opened)
    |> Array.map (fun v -> v.Label)
    |> Set.ofArray

unopenedValves |> pp

let endtime = 30

let rec traverse n (state: State) (valve: string) openValve =
    let info = valvesDict[valve]

    let nextState =
        match openValve with
        | true ->
            { state with
                Pressure = state.Pressure + state.Rate
                Rate = state.Rate + info.FlowRate
                LastRateChange = 0
                Path = valve + "*" :: state.Path
                Rates = state.Rate :: state.Rates
                Unopened = Set.remove valve state.Unopened }
        | _ ->

            let pathItem =
                match state.Unopened.Contains valve with
                | _ when openValve -> valve + "*"
                | _ -> valve

            { state with
                Pressure = state.Pressure + state.Rate
                LastRateChange = state.LastRateChange + 1
                Path = pathItem :: state.Path
                Rates = state.Rate :: state.Rates }

    // Detect loop w/o rate change
    // Check if valve is in path, if so, check if last rate change is greater than last appearance of valve in path
    let loopFound =
        List.contains valve state.Path
        && state.LastRateChange > 4
        && (List.findIndex (fun v -> v = valve) state.Path < state.LastRateChange)

    match n with
    | n when n = endtime -> nextState
    //| _ when state.LastRateChange = 4 -> nextState
    | _ when loopFound -> nextState
    | _ when nextState.Unopened.IsEmpty -> { nextState with Pressure = state.Pressure + (endtime - n) * state.Rate }
    | _ ->
        let tunnelMax =
            valvesDict[valve].Tunnels
            |> Array.map (fun v -> traverse (n + 1) nextState v false)
            |> Array.maxBy (fun v -> v.Pressure)
        // if valve is unopened, add option for traverse
        match nextState.Unopened.Contains valve with
        | false -> tunnelMax
        | _ ->
            let openValveTunnelState = (traverse (n + 1) nextState valve true)

            match tunnelMax.Pressure < openValveTunnelState.Pressure with
            | true -> openValveTunnelState
            | _ -> tunnelMax


let initialState =
    { Pressure = 0
      Rate = 0
      Unopened = unopenedValves
      LastRateChange = 0
      Path = []
      Rates = [] }

timeOperation (fun () -> traverse 0 initialState "AA" false)
|> pp
