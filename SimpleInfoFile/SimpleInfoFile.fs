namespace SimpleInfoFile.Realization


module SimpleInfoFile = 
    open System.IO

    type InfoFile = 
        {
            CountLines: int
            CountAsciiSymbols: int
            SizeInBytesAscii: int 
        }

    type InfoData = string list -> int

    let countLine: InfoData =
        List.length

    ///Посчитать количество печатаемых символов в ASCII тексте
    let countAsciiSymbols: InfoData =
        let isPrintableAscii (x: char): bool = x >= '\x20' && x <= '\x7e'
        fun y -> y |> List.map (fun x -> x |> String.filter isPrintableAscii |> String.length) |> List.sum
    
    ///Посчитать размер ASCII текста в байтах
    let calcSizeDataInBytesAscii: InfoData =
        let sizeAscii x = x * 1
        countAsciiSymbols >> sizeAscii

    let getInfoFile (path: string): InfoFile = 
        let read = [
            use sr = new StreamReader (path)
            while not sr.EndOfStream do
                yield sr.ReadLine()
        ]
        let data = read
        let result = 
            {
                CountLines = countLine data
                CountAsciiSymbols = countAsciiSymbols data
                SizeInBytesAscii = calcSizeDataInBytesAscii data
            }
        result
