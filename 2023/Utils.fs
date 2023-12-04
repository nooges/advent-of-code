namespace AOC

open System
open System.Diagnostics
open System.Text.RegularExpressions
open System.Collections.Generic

module Utils =

    let split c (s: string) = s.Split([| c |])
    let splitStr (c: string) (s: string) = s.Split(c)
    let contains (c: char) (s: string) = s.Contains(c)
    let trim (s: string) = s.Trim()
    let replace (a: string) (b: string) (s: string) = s.Replace(a, b)
    let asCharArray (s: string) = s.ToCharArray()
    let asStringArray: string[] -> string[] = Array.map string
    let asIntArray: string[] -> int[] = Array.map int
    let charToInt (c: char) = int (c) - int 'a'
    let charDigitToInt (c: char) = int (c) - int '0'
    let pp a = printfn "%A" a

    let pp1 a =
        printfn "Part 1:\n-------\n%A\n-------" a

    let pp2 a =
        printfn "Part 2:\n-------\n%A\n-------" a

    let ps s = printfn "%A" (s |> Seq.toArray)

    let withRegex regex str =
        [| for m in Regex.Match(str, regex).Groups -> m.Value |] |> Array.tail

    let allInts str =
        [| for m in Regex.Match(str, @"-?\d+").Groups -> m.Value |]
        |> Array.tail
        |> Array.map int

    let extractInts (input: string) =
        let pattern = @"-?\d+"
        let regex = new Regex(pattern)
        let matches = regex.Matches(input)

        [| for m in matches do
               yield int m.Value |]

    let isBetween lower upper n = n >= lower && n <= upper

    type Point = { x: int; y: int }

    type TimedOperation<'T> =
        { millisecondsTaken: int64
          returnedValue: 'T }

    let timeOperation<'T> (func: unit -> 'T) : TimedOperation<'T> =
        let timer = new Stopwatch()
        timer.Start()
        let returnValue = func ()
        timer.Stop()

        { millisecondsTaken = timer.ElapsedMilliseconds
          returnedValue = returnValue }

    let memoize f =
        let dict = Dictionary<_, _>()

        fun c ->
            let exist, value = dict.TryGetValue c

            match exist with
            | true -> value
            | _ ->
                let value = f c
                dict.Add(c, value)
                value
