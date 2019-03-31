namespace SimpleInfoFile.Realization


module SimpleInfoFile = 

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
        let isPrintableAscii (x: char): bool = x >= '\x31' && x <= '\x7e'
        fun y -> y |> List.map (fun x -> x |> String.filter isPrintableAscii |> String.length) |> List.sum
    
    ///Посчитать размер ASCII текста в байтах
    let calcSizeDataInBytesAscii: InfoData =
        let sizeAscii x = x * 1
        countAsciiSymbols >> sizeAscii

    let getInfoFile (path: string): InfoFile = 
        { 
            CountLines = 0
            CountAsciiSymbols = 0
            SizeInBytesAscii = 0
        }
