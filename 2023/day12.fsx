#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

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

let makePossibleArrangements (condition: string) =
    let rec loop acc l =
        match l with
        | [] -> [ acc ]
        | '?' :: cs -> loop (acc + ".") cs @ loop (acc + "#") cs
        | c :: cs -> loop (acc + string c) cs

    loop "" (condition.ToCharArray() |> Array.toList)

let conditionGroups (condition: string) =
    let rec loop acc curr l =
        match (l, curr) with
        | ([], 0) -> acc
        | ([], _) -> curr :: acc
        | ('#' :: cs, _) -> loop acc (curr + 1) cs
        | ('.' :: cs, 0) -> loop acc 0 cs
        | ('.' :: cs, _) -> loop (curr :: acc) 0 cs
        | _ -> acc // Shouldn't hit this

    loop [] 0 (condition.ToCharArray() |> Array.rev |> Array.toList)

//".??..??...?##." |> makePossibleArrangements |> pp
//"?????..?#??#?#??###?" |> makePossibleArrangements |> pp

//"...##.....#..#.##.###." |> conditionGroups |> pp
timeOperation (fun () ->
    records
    |> Array.sumBy (fun r ->
        r.condition
        |> makePossibleArrangements
        |> List.map conditionGroups
        |> List.filter (fun g -> g = r.groups)
        |> List.length))
|> pp1
