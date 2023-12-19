#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"

open AOC.Utils

let input = System.IO.File.ReadAllText("data/day19-input.txt")

type Rule =
    { cat: string
      gt: bool
      v: int
      dst: string }

type Workflow = { rules: Rule list; otherwise: string }

let data = input |> splitStr "\n\n"

let ratings =
    data[1]
    |> split '\n'
    |> Array.map (withRegex "{x=(.*),m=(.*),a=(.*),s=(.*)}" >> Array.map int)
    |> Array.map (fun a -> Map [ "x", a[0]; "m", a[1]; "a", a[2]; "s", a[3]; "total", Array.sum a ])

let parseRule (s: string) =
    s
    |> withRegex "([xmas])([<>])(\d+):(.*)"
    |> (fun a ->
        { cat = a[0]
          gt = a[1] = ">"
          v = int a[2]
          dst = a[3] })

let parseWorkflow (s: string[]) =
    (s[0],
     { rules = s[1] |> split ',' |> Array.map parseRule |> Array.toList
       otherwise = s |> Array.last })

let workflows =
    data[0]
    |> split '\n'
    |> Array.map (withRegex "(.*){(.*),(.*)}" >> parseWorkflow)
    |> Map.ofArray

let rec nextDst rules (part: Map<string, int>) =
    match rules with
    | [] -> None
    | x :: xs ->
        match x.gt with
        | true -> if part[x.cat] > x.v then Some(x.dst) else nextDst xs part
        | _ -> if part[x.cat] < x.v then Some(x.dst) else nextDst xs part

let isAccepted part =
    let rec loop dst =
        match dst with
        | "A" -> true
        | "R" -> false
        | _ ->
            let wf = workflows[dst]

            match nextDst wf.rules part with
            | None -> loop wf.otherwise
            | Some(d) -> loop d

    loop "in"

timeOperation (fun () -> ratings |> Array.filter isAccepted |> Array.sumBy (fun m -> m["total"]))
|> pp1
