module Tests

open Expecto
open SimpleInfoFile.Realization.SimpleInfoFile

[<Tests>]
let tests =
      let commonData = ["123"; "1234"; "test"]
      testList "примеры" [
        test "посчитать количество строк" {
            let actual = countLine commonData
            let expected = 3
            Expect.equal actual expected |> ignore
        }    
        test "посчитать количество байт" {
            let actual = calcSizeDataInBytes commonData
            let expected = 3
            Expect.equal actual expected |> ignore
        }    
        test "посчитать количество ASCII символов" {
            let actual = countAsciiSymbols commonData
            let expected = 3
            Expect.equal actual expected |> ignore
        }
        test "получить инфу о файле" {
            let actual = getInfoFile "test.txt"
            let expected =         
                { 
                    CountLines = 0
                    CountAsciiSymbols = 0
                    SizeInBytes = 0
                }
            Expect.equal actual expected |> ignore
        }
      ]
