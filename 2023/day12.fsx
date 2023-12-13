#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils
open System.Collections.Generic

let input = System.IO.File.ReadAllLines("data/day12-sample.txt")

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

let makePossibleArrangements totalSprings groups (condition: string) =

    let rec loop springs (l: string) =
        let hash = (springs, l)

        let exist, value = cache.TryGetValue hash

        match exist with
        | true ->
            //pp ("cache hit", springs, l)
            value
        | _ ->
            //let possibleSpringsLeft =
            //    l |> asCharArray |> Array.filter (fun c -> c = '#' || c = '?') |> Array.length
            let value =
                match l with
                | "" -> [ "" ]
                | _ when l.StartsWith("#") ->
                    loop (springs + 1) l[1..]
                    |> List.map (fun s -> "#" + s)
                    |> List.filter (fun s -> conditionGroups s = groups)
                | _ when l.StartsWith(".") ->
                    loop springs l[1..]
                    |> List.map (fun s -> "." + s)
                    |> List.filter (fun s -> conditionGroups s = groups)
                | _ ->
                    (loop (springs + 1) l[1..] |> List.map (fun s -> "#" + s))
                    @ (loop springs l[1..] |> List.map (fun s -> "." + s))
                    |> List.filter (fun s -> conditionGroups s = groups)

            cache.Add(hash, value)
            value

    loop 0 condition

//"...##.....#..#.##.###." |> conditionGroups |> pp
timeOperation (fun () ->
    records
    |> Array.sumBy (fun r ->
        r.condition
        |> makePossibleArrangements (r.groups |> List.sum) r.groups
        |> List.length))
|> pp1

records |> pp

let copies = 1

(*
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
*)
