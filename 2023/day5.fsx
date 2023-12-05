#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day5-input.txt")

let dataGroups = input |> (fun x -> x.Split("\n\n"))
let seeds = dataGroups[0] |> allInt64s

type categoryMapRange = { dst: int64; src: int64; len: int64 }

type categoryMap =
    { name: string
      maps: categoryMapRange[] }

let parseCategoryMap =
    split '\n'
    >> (fun a ->
        { name = a[0]
          maps =
            a
            |> Array.tail
            |> Array.map (allInt64s >> fun a -> { dst = a[0]; src = a[1]; len = a[2] }) })

let maps = dataGroups |> Array.tail |> Array.map parseCategoryMap

let mapToNewCategory n (cm: categoryMap) =
    cm.maps
    |> Array.tryPick (fun m ->
        match n >= m.src && n < (m.src + m.len) with
        | true -> Some(n + m.dst - m.src)
        | _ -> None)
    |> Option.defaultValue n

seeds
|> Array.map (fun s -> maps |> Array.fold mapToNewCategory s)
|> Array.min
|> pp1
