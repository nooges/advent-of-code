#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

type Blueprint =
    { Number: int
      OreBotCost: int
      ClayBotCost: int
      ObsidianBotCost: int []
      GeodeBotCost: int [] }

type State =
    { Time: int
      Ore: int
      Clay: int
      Obsidian: int
      Geodes: int
      OreBots: int
      ClayBots: int
      ObsidianBots: int
      GeodeBots: int }

let input = System.IO.File.ReadAllLines("data/day19-input.txt")

let parseLine line =
    line
    |> withRegex
        "Blueprint (.*): Each ore robot costs (.*) ore. Each clay robot costs (.*) ore. Each obsidian robot costs (.*) ore and (.*) clay. Each geode robot costs (.*) ore and (.*) obsidian."
    |> (fun x ->
        { Number = int x[0]
          OreBotCost = int x[1]
          ClayBotCost = int x[2]
          ObsidianBotCost = [| int x[3]; int x[4] |]
          GeodeBotCost = [| int x[5]; int x[6] |] })

let endTime = 24

let rec dfs (state: State) (blueprint: Blueprint) =
    match state.Time with
    | n when n = endTime -> state
    | n when n = endTime - 1 && state.GeodeBots = 0 -> state
    | _ ->
        let state' =
            { state with
                Time = state.Time + 1
                Ore = state.Ore + state.OreBots
                Clay = state.Clay + state.ClayBots
                Obsidian = state.Obsidian + state.ObsidianBots
                Geodes = state.Geodes + state.GeodeBots }

        let mutable newStates = [ dfs state' blueprint ]

        if state.Ore >= blueprint.GeodeBotCost[0]
           && state.Obsidian >= blueprint.GeodeBotCost[1] then
            let testState =
                dfs
                    { state' with
                        Ore = state'.Ore - blueprint.GeodeBotCost[0]
                        Obsidian = state'.Obsidian - blueprint.GeodeBotCost[1]
                        GeodeBots = state.GeodeBots + 1 }
                    blueprint

            if testState.Time = endTime then
                newStates <- [ testState ]

        else
            if state.Ore >= blueprint.ObsidianBotCost[0]
               && state.Clay >= blueprint.ObsidianBotCost[1] then
                let testState =
                    dfs
                        { state' with
                            Ore = state'.Ore - blueprint.ObsidianBotCost[0]
                            Clay = state'.Clay - blueprint.ObsidianBotCost[1]
                            ObsidianBots = state.ObsidianBots + 1 }
                        blueprint

                if testState.Time = endTime then
                    newStates <- testState :: newStates

            if state.Ore >= blueprint.ClayBotCost then
                let testState =
                    dfs
                        { state' with
                            Ore = state'.Ore - blueprint.ClayBotCost
                            ClayBots = state.ClayBots + 1 }
                        blueprint

                if testState.Time = endTime then
                    newStates <- testState :: newStates

            if state.Ore >= blueprint.OreBotCost then
                let testState =
                    dfs
                        { state' with
                            Ore = state'.Ore - blueprint.OreBotCost
                            OreBots = state.OreBots + 1 }
                        blueprint

                if testState.Time = endTime then
                    newStates <- testState :: newStates

        newStates |> List.maxBy (fun s -> s.Geodes)

let memDfs = memoize dfs

let blueprints = input |> Array.map parseLine


let initialState =
    { Time = 0
      Ore = 0
      Clay = 0
      Obsidian = 0
      Geodes = 0
      OreBots = 1
      ClayBots = 0
      ObsidianBots = 0
      GeodeBots = 0 }



timeOperation (fun () ->
    blueprints
    |> Array.map (fun b ->
        let res = b.Number * (memDfs initialState b).Geodes
        res |> pp
        res)
    |> Array.sum)
|> pp1

(*

timeOperation (fun () ->
    blueprints
    |> Array.take 3
    |> Array.map (fun b ->
        let res = (memDfs initialState b).Geodes
        res |> pp
        res)
    |> Array.reduce (*))
|> pp2
*)
