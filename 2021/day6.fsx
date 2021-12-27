#!/usr/bin/env -S dotnet fsi

open System.IO

let inputLine = File.ReadLines("data/day6-input.txt")
let regenTime = 6
let newFishTime = 8

let updateFishTimer timer =
    match timer with
    | 0 -> regenTime
    | _ -> timer - 1

let part1 =
    let initialFishTimers =
        inputLine
        |> Seq.head
        |> (fun x -> x.Split ',')
        |> Array.map int

    let numDays = 80

    let rec trackFish days fishTimers =
        //printfn "%d: %A" days fishTimers

        let newFish =
            fishTimers
            |> Array.filter (fun f -> f = 0)
            |> Array.length

        let newFishTimers =
            Array.concat [| fishTimers
                            Array.create newFish (newFishTime + 1) |]

        match days with
        | 1 -> newFishTimers
        | _ -> trackFish (days - 1) (newFishTimers |> Array.map updateFishTimer)

    trackFish numDays initialFishTimers
    |> Array.length

let updateCounts (counts: bigint array) =
    let numNewFish = counts.[0]

    let newCounts =
        Array.concat [| counts.[1..8]
                        [| numNewFish |] |]

    newCounts.[6] <- newCounts.[6] + numNewFish
    newCounts

let part2 =
    let fishCounts = Array.create (newFishTime + 1) 0I
    let numDays = 256

    inputLine
    |> Seq.head
    |> (fun x -> x.Split ',')
    |> Array.map int
    |> Array.iter (fun x -> fishCounts.[x] <- fishCounts.[x] + 1I)


    let rec trackFish days counts =
        printfn "%d: %A" days counts

        match days with
        | 0 -> counts
        | _ -> trackFish (days - 1) (updateCounts counts)

    trackFish numDays fishCounts |> Array.sum


printfn "Part 1: %A" part1
printfn "Part 2: %A" part2
