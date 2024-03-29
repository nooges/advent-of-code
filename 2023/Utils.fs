namespace AOC

open System
open System.Diagnostics
open System.Text.RegularExpressions
open System.Collections.Generic

module Utils =

    let split c (s: string) = s.Split([| c |])
    let splitNoEmpty (c: char) (s: string) = s.Split([| c |], StringSplitOptions.RemoveEmptyEntries)
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
    // Tee operator
    let (|>!) x sideEffect =
        sideEffect x |> ignore
        x

    let pp1 a =
        printfn "Part 1:\n-------\n%A\n-------" a

    let pp2 a =
        printfn "Part 2:\n-------\n%A\n-------" a

    let ps s = printfn "%A" (s |> Seq.toArray)

    let withRegex regex str =
        [| for m in Regex.Match(str, regex).Groups -> m.Value |] |> Array.tail

    let allInts = Regex(@"-?\d+").Matches >> Seq.map (_.Value >> int) >> Array.ofSeq

    let allInt64s = Regex(@"-?\d+").Matches >> Seq.map (_.Value >> int64) >> Array.ofSeq

    let isBetween lower upper n = n >= lower && n <= upper

    type Point = { x: int; y: int }
    type GridPoint = { r: int; c: int }

    type TimedOperation<'T> = { ms: float; result: 'T }

    let timeOperation<'T> (func: unit -> 'T) : TimedOperation<'T> =
        let timer = new Stopwatch()
        timer.Start()
        let result = func ()
        timer.Stop()

        { ms = (float timer.ElapsedTicks) / (float Stopwatch.Frequency) * 1000.
          result = result }

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
