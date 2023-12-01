#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

type Tree<'LeafData, 'INodeData> =
    | LeafNode of 'LeafData
    | InternalNode of 'INodeData * Tree<'LeafData, 'INodeData> seq

type FileInfo = { name: string; size: int }
type DirInfo = { name: string }
type FSItem = Tree<FileInfo, DirInfo>

module Tree =

    let rec cata fLeaf fNode (tree: Tree<'LeafData, 'INodeData>) : 'r =
        let recurse = cata fLeaf fNode

        match tree with
        | LeafNode leafInfo -> fLeaf leafInfo
        | InternalNode (nodeInfo, subtrees) -> fNode nodeInfo (subtrees |> Seq.map recurse)

    let rec fold fLeaf fNode acc (tree: Tree<'LeafData, 'INodeData>) : 'r =
        let recurse = fold fLeaf fNode

        match tree with
        | LeafNode leafInfo -> fLeaf acc leafInfo
        | InternalNode (nodeInfo, subtrees) ->
            // determine the local accumulator at this level
            let localAccum = fNode acc nodeInfo
            // thread the local accumulator through all the subitems using Seq.fold
            let finalAccum = subtrees |> Seq.fold recurse localAccum
            // ... and return it
            finalAccum

let fromFile (fileInfo: FileInfo) = LeafNode fileInfo
let fromDir (dirInfo: DirInfo) files = InternalNode(dirInfo, files)

let input = System.IO.File.ReadAllLines("data/day7-sample.txt")

let parseLine (line: string) : (FSItem) =
    match line with
    | line when line.StartsWith("$ cd") -> fromFile { name = "cd"; size = 0 }
    | line when line.StartsWith("$ ls") -> fromFile { name = "ls"; size = 0 }
    | line when line.StartsWith("dir") -> fromDir { name = "dir" } []
    | line ->
        let l = line.Split(' ')
        fromFile { name = l[1]; size = int (l[0]) }

let parseInput: (FSItem) =
    let initialFs = fromDir { name = "/" } []

    let rec loop fs lines =
        match lines with
        | [] -> fs
        | x :: xs -> loop fs xs

    loop initialFs (input |> Array.toList)

input |> pp
parseInput |> pp
