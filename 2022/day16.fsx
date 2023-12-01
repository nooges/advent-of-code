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

let endtime = 26

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
                | _ when openValve -> valve + "-"
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
        && state.LastRateChange > 7
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


let rec traverse2 n (state: State) (v1: string) (v2: string) o1 o2 =
    let info1 = valvesDict[v1]
    let info2 = valvesDict[v2]

    let nextState =
        match (o1, o2) with
        | (true, true) ->
            { state with
                Pressure = state.Pressure + state.Rate
                Rate = state.Rate + info1.FlowRate + info2.FlowRate
                LastRateChange = 0
                Path = v2 + "*" + v1 + "*" :: state.Path
                Rates = state.Rate :: state.Rates
                Unopened = state.Unopened.Remove(v1).Remove(v2) }
        | (true, false) ->
            { state with
                Pressure = state.Pressure + state.Rate
                Rate = state.Rate + info1.FlowRate
                LastRateChange = 0
                Path = v2 + v1 + "*" :: state.Path
                Rates = state.Rate :: state.Rates
                Unopened = state.Unopened.Remove(v1) }
        | (false, true) ->
            { state with
                Pressure = state.Pressure + state.Rate
                Rate = state.Rate + info2.FlowRate
                LastRateChange = 0
                Path = v2 + "*" + v1 :: state.Path
                Rates = state.Rate :: state.Rates
                Unopened = state.Unopened.Remove(v2) }
        | _ ->

            let pathItem1 =
                match state.Unopened.Contains v1 with
                | _ when o1 -> v1 + "*"
                | _ -> v1

            let pathItem2 =
                match state.Unopened.Contains v2 with
                | _ when o2 -> v2 + "*"
                | _ -> v2

            { state with
                Pressure = state.Pressure + state.Rate
                LastRateChange = state.LastRateChange + 1
                Path = pathItem2 + pathItem1 :: state.Path
                Rates = state.Rate :: state.Rates }

    // Detect loop w/o rate change
    // Check if valve is in path, if so, check if last rate change is greater than last appearance of valve in path
    let loopFound =
        (List.contains v1 state.Path
         && state.LastRateChange > 3
         && (List.findIndex (fun v -> v = v1) state.Path < state.LastRateChange))
        || (List.contains v2 state.Path
            && state.LastRateChange > 3
            && (List.findIndex (fun v -> v = v2) state.Path < state.LastRateChange))

    match n with
    | n when n = endtime -> nextState
    | _ when loopFound -> nextState
    | _ when nextState.Unopened.IsEmpty -> { nextState with Pressure = state.Pressure + (endtime - n) * state.Rate }
    | _ ->
        let tunnelMax =
            valvesDict[v1].Tunnels
            |> Array.map (fun v ->
                valvesDict[v2].Tunnels
                |> Array.map (fun w -> traverse2 (n + 1) nextState v w false false)
                |> Array.maxBy (fun v -> v.Pressure))
            |> Array.maxBy (fun v -> v.Pressure)
        // if valve is unopened, add option for traverse
        let openStuff = [| tunnelMax |]

        if nextState.Unopened.Contains v1 then
            Array.append [| traverse2 (n + 1) nextState v1 |]

        match nextState.Unopened.Contains v1 with
        | false -> tunnelMax
        | _ ->
            let openValveTunnelState = (traverse2 (n + 1) nextState valve true)

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

let initialState2 =
    { Pressure = 0
      Rate = 0
      Unopened =
        set [ "CE"
              "GV"
              "OU"
              "RG"
              "TM"
              "UO"
              "XG"
              "ZB" ]
      LastRateChange = 0
      Path = []
      Rates = [] }

//timeOperation (fun () -> traverse 0 initialState2 "AA" false)
//|> pp
