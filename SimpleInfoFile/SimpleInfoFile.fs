namespace SimpleInfoFile.Realization


module SimpleInfoFile = 

    type InfoFile = 
        {
            CountLines: int
            CountAsciiSymbols: int
            SizeOfByte: int 
        }

    let getInfoFile (path: string): InfoFile = 
        { 
            CountLines = 0
            CountAsciiSymbols = 0
            SizeOfByte = 0
        }
