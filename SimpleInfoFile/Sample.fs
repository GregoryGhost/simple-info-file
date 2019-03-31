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
        test "посчитать количество байт ASCII" {
            let actual = calcSizeDataInBytes commonData
            let expected = 11
            Expect.equal actual expected |> ignore
        }
        test "посчитать количество ASCII символов" {
            let actual = countAsciiSymbols commonData
            let expected = 11
            Expect.equal actual expected |> ignore
        }
        test "получить инфу о файле" {
            let actual = getInfoFile "test.txt"
            let expected =         
                { 
                    CountLines = 3
                    CountAsciiSymbols = 14
                    SizeInBytesAscii = 14
                }
            Expect.equal actual expected |> ignore
        }
      ]
