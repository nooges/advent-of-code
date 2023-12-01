#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
#r "nuget: FSharp.Data, 5.0.2"

open AOC.Utils
open FSharp.Data
open FSharp.Data.JsonExtensions

type Comparisons =
| Equal
| Lesser
| Greater

let input = System.IO.File.ReadAllText("data/day13-sample.txt")

let sample = "[1,[2,[3,[4,[5,6,7]]]],8,9]"
let sample3 = "[1,1,3,1,1]"

let sample2 =
    "[[4],[2,[10,[5,7,8,7,0]],[4,8,[1,2],[5]],3,9],[[[3,3,3,5,4],5,[],7,[7,3,10,4,0]],9,[3]],[2,0,6,[9,5],8],[[4,[9,8,6],[],5],3,[7,7,[3,3,6],7,[9,4,0,10,6]],10,[]]]"

let compareJsonValue (a: JsonValue) (b: JsonValue) =
    match JsonConversions.



input.Split("\n\n")
|> Array.mapi (fun i x -> (i + 1, x |> split '\n' |> Array.map JsonValue.Parse))
|> pp


let test = JsonValue.Parse sample2 |> pp
