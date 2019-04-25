module Tests

open Expecto
open SimpleInfoFile.Realization.SimpleInfoFile
open System.Text

[<Tests>]
let tests =
      testList "примеры" [
        test "получить инфу о файле" {
            let actual = getInfoFile "test.txt"
            let expected =         
                { 
                    CountLines = 3
                    CountAsciiSymbols = 14
                    SizeInBytes = 21
                }
            Expect.equal actual expected "не совпало"

            let expected2 = new System.IO.FileInfo("test.txt")
            Expect.equal (actual.SizeInBytes |> int64) expected2.Length "не совпало с IO.FileInfo"
        }
      ]
