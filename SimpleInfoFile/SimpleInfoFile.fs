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
        fun x -> 3

    let calcSizeDataInBytes: InfoData =
        fun x -> 3

    let countAsciiSymbols: InfoData =
        fun x -> 3

    let getInfoFile (path: string): InfoFile = 
        { 
            CountLines = 0
            CountAsciiSymbols = 0
            SizeInBytesAscii = 0
        }
