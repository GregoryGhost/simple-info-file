module Tests

open Expecto
open SimpleInfoFile.Realization.SimpleInfoFile
open System.Text

[<Tests>]
let tests =
      let toAscii (x: string list): string list =
        let encoder = new ASCIIEncoding()
        let toBytes (y: string): byte [] = y.ToCharArray() |> Array.map (byte)
        x |> List.map (toBytes >> encoder.GetString)
      let commonData = ["1235"; "123"; "test\x7f"] |> toAscii

      testList "примеры" [
        test "посчитать количество строк" {
            let actual = countLine commonData
            let expected = 3
            Expect.equal actual expected "не совпало"
        }
        test "посчитать количество байт ASCII" {
            let actual = calcSizeDataInBytesAscii commonData
            let expected = 11
            Expect.equal actual expected "не совпало"
        }
        test "посчитать количество ASCII символов" {
            let actual = countAsciiSymbols commonData
            let expected = 11
            Expect.equal actual expected "не совпало"
        }
        test "получить инфу о файле" {
            let actual = getInfoFile "test.txt"
            let expected =         
                { 
                    CountLines = 3
                    CountAsciiSymbols = 14
                    SizeInBytesAscii = 14
                }
            Expect.equal actual expected "не совпало"
        }
      ]
