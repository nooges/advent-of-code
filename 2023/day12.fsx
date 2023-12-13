#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Collections.Generic

let input = System.IO.File.ReadAllLines("data/day12-input.txt")

type SpringRecord = { condition: string; groups: int list }

let records =
    input
    |> Array.map (
        split ' '
        >> (fun a ->
            { condition = a[0]
              groups = a[1] |> split ',' |> Array.map int |> Array.toList })
    )



let cache = System.Collections.Generic.Dictionary<_, _>()

let makePossibleArrangements totalSprings (condition: string) =


    // TODO: move acc to outside and with output of loop, map/concat the list to acc
    // TODO: loop cache args: # possible springs left, remaining string
    let rec loop acc springs l =
        let hash = (acc, springs, l)

        let exist, value = cache.TryGetValue hash

        match exist with
        | true -> value
        | _ ->
            let possibleSpringsLeft =
                l |> List.filter (fun c -> c = '#' || c = '?') |> List.length

            let value =
                match l with
                | [] -> [ acc ]
                | _ when springs + possibleSpringsLeft < totalSprings -> []
                | '?' :: cs -> loop (acc + ".") springs cs @ loop (acc + "#") (springs + 1) cs
                | '#' :: cs -> loop (acc + "#") (springs + 1) cs
                | c :: cs -> loop (acc + ".") springs cs

            cache.Add(hash, value)
            value

    loop "" 0 (condition.ToCharArray() |> Array.toList)

let conditionGroups (condition: string) =
    let rec loop acc curr l =
        match (l, curr) with
        | ([], 0) -> acc
        | ([], _) -> curr :: acc
        | ('#' :: cs, _) -> loop acc (curr + 1) cs
        | ('.' :: cs, 0) -> loop acc 0 cs
        | ('.' :: cs, _) -> loop (curr :: acc) 0 cs
        | _ ->
            (printfn "bad") |> ignore
            acc // Shouldn't hit this

    loop [] 0 (condition.ToCharArray() |> Array.rev |> Array.toList)

".??..??...?##." |> makePossibleArrangements 5 |> pp

//"...##.....#..#.##.###." |> conditionGroups |> pp
timeOperation (fun () ->
    records
    |> Array.sumBy (fun r ->
        r.condition
        |> makePossibleArrangements (r.groups |> List.sum)
        |> List.map conditionGroups
        |> List.filter (fun g -> g = r.groups)
        |> List.length))
|> pp1

records |> pp

let copies = 2

timeOperation (fun () ->
    records
    |> Array.sumBy (fun r ->
        r.condition
        |> List.replicate copies
        |> String.concat "?"
        |> makePossibleArrangements (r.groups |> List.sum |> (*) copies)
        |> List.map conditionGroups
        |> List.filter (fun g -> g = (r.groups |> List.replicate copies |> List.concat))
        |> List.length
        |>! pp))
|> pp2
