namespace AOC

open System

module Utils =

    let split c (s: string) = s.Split([| c |])
    let replace (a: string) (b: string) (s: string) = s.Replace(a, b)
