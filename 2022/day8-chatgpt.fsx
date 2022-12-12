#!/usr/bin/env -S dotnet fsi

#load "Utils.fs"
open AOC.Utils

// Parse the input string into a 2D array of integers
let parseInput input =
    input
    |> split '\n'
    |> Array.map (fun row -> row |> Seq.toArray |> Array.map int)

// Count the number of trees visible from outside the grid
let countVisibleTrees input =
    let map = parseInput input

    // Get the maximum height of trees in the given direction
    let maxHeight (row: int) (col: int) (dRow: int) (dCol: int) =
        let rec loop (row: int) (col: int) (maxHeight: int) =
            if row < 0
               || row >= map.Length
               || col < 0
               || col >= map.[0].Length then
                // We have reached an edge of the grid, so we return the maximum height
                // that we have seen so far
                maxHeight
            else
                // Update the maximum height and continue moving in the given direction
                let height = map.[row].[col]

                let newMaxHeight =
                    if height > maxHeight then
                        height
                    else
                        maxHeight

                loop (row + dRow) (col + dCol) newMaxHeight

        // Start moving in the given direction from the given starting position
        loop row col 0

    // Count the number of trees that are visible in the given direction
    let countVisibleInDirection (dRow: int) (dCol: int) =
        let rec loop (row: int) (col: int) (count: int) =
            if row < 0
               || row >= map.Length
               || col < 0
               || col >= map.[0].Length then
                // We have reached an edge of the grid, so we return the number of trees
                // that we have counted so far
                count
            else
                // Update the number of counted trees and continue moving in the given
                // direction
                let height = map.[row].[col]
                let maxHeight = maxHeight row col dRow dCol

                let newCount =
                    if height = maxHeight then
                        count + 1
                    else
                        count

                loop (row + dRow) (col + dCol) newCount

        // Start moving in the given direction from the upper-left corner of the grid
        loop 0 0 0

    // Count the number of trees that are visible from the top, bottom, left, and
    // right of the grid
    let countTop = countVisibleInDirection (-1) 0
    let countBottom = countVisibleInDirection 1 0
    let countLeft = countVisibleInDirection 0 (-1)
    let countRight = countVisibleInDirection 0 1

    // Return the total number of visible trees
    countTop + countBottom + countLeft + countRight

// Test the countVisibleTrees function
let input =
    "30373
   25512
   65332
   33549
   35390"

let expectedOutput = 21
let actualOutput = countVisibleTrees input
printfn "Expected: %d, actual: %d" expectedOutput actualOutput
