type Game =
    { gameId: int
      samples: Map<string, int> list }

let headToNumber (stringForm: string) = stringForm.Trim().Split()[1] |> int

let comboToMapItem (stringForm: string) =
    match stringForm.Trim().Split() |> Array.toList with
    | amount :: colour -> (colour.Head, amount |> int)
    | _ -> failwith "Could not parse input to games"

let sampleToMap (stringForm: string) =
    stringForm.Trim().Split(",") |> Array.map comboToMapItem |> Map.ofArray

let samplesToMap (stringForm: string) =
    stringForm.Split(";") |> Array.map sampleToMap |> Array.toList

let stringToGame (stringForm: string) =
    match stringForm.Split ":" |> Array.toList with
    | gameTag :: rest ->
        { gameId = headToNumber gameTag
          samples = (rest.Head |> samplesToMap) }
    | _ -> failwith "Could not parse input to games"

let maximumNumbers = Map [ ("red", 12); ("green", 13); ("blue", 14) ]

let onlyBelow = Map.forall (fun key value -> maximumNumbers[key] >= value)

let calculateAcceptableSum =
    List.filter (fun game -> game.samples |> List.forall onlyBelow)
    >> List.map (fun game -> game.gameId)
    >> List.sum

let absoluteMinimum = Map [ ("red", 0); ("green", 0); ("blue", 0) ]

let mergeMaps =
    Map.fold (fun acc key value ->
        match Map.tryFind key acc with
        | Some(existingValue) -> Map.add key (max existingValue value) acc
        | None -> Map.add key value acc)

let ProductOfMinima =
    List.fold mergeMaps absoluteMinimum
    >> Map.fold (fun acc _key value -> acc * value) 1

let calculateSumOfProductOfMinima =
    List.map (fun game -> ProductOfMinima game.samples) >> List.sum

let runStars args =
    printfn "Game 1: %A" (calculateAcceptableSum args)
    printfn "Game 2: %A" (calculateSumOfProductOfMinima args)

let readLines = System.IO.File.ReadLines >> Seq.cast<string> >> List.ofSeq

[<EntryPoint>]
let main args =
    match args |> Array.toList with
    | [ a ] -> a |> readLines |> List.map stringToGame |> runStars
    | _ -> failwith "Could not read commandline argument as file name"

    0
