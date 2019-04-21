namespace SimpleInfoFile.Realization


module SimpleInfoFile = 
    open System.IO

    type InfoFile = 
        {
            CountLines: int
            CountAsciiSymbols: int
            SizeInBytes: int 
        }

    type InfoData = byte[] -> int

    let private countLine: InfoData =
        let inc x = x + 1
        Array.where (fun x -> x = 10uy) >> Array.length >> inc

    ///Посчитать количество печатаемых символов в ASCII тексте
    let private countAsciiSymbols: InfoData =
        let isPrintableAscii (x: byte): bool = 
            if x >= 32uy && x <= 126uy then
                true
            else
                false
        Array.filter (isPrintableAscii) >> Array.length
    
    ///Посчитать размер текста в байтах
    let private calcSizeDataInBytes: InfoData =
        Array.length

    let getInfoFile (path: string): InfoFile = 
        let data =
            File.ReadAllBytes(path)
        let result = 
            {
                CountLines = countLine data
                CountAsciiSymbols = countAsciiSymbols data
                SizeInBytes = calcSizeDataInBytes data
            }
        result
