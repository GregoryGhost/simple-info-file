pub mod simple_info_file {

    #[derive(Debug)]
    pub struct InfoFile {
        pub lines: usize,
        pub printable_ascii_symbols: usize,
        pub size_in_bytes_ascii: usize,
    }


    fn count_ascii_chars(data: &[&str], _callback: fn(char) -> bool) -> usize {
        let mut acc_ascii: usize = 0;
        for x in data {
            acc_ascii += x.chars().fold(0, |acc, i| match _callback(i) {
                true => acc + 1,
                false => acc,
            });
        }
        acc_ascii
    }

    pub fn count_line(data: &[&str]) -> usize {
        data.len()
    }

    pub fn count_printable_ascii_symbols(data: &[&str]) -> usize {
        count_ascii_chars(data, |i| '\x20' <= i && i <= '\x7E')
    }

    pub fn calc_size_data_in_ascii_bytes(data: &[&str]) -> usize {
        count_ascii_chars(data, |i| i.is_ascii())
    }

    use std::fs::File;
    use std::io::Read;
    use std::str;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn get_info_text(text: &[&str]) -> std::io::Result<InfoFile> {
        let info = InfoFile {
            lines: count_line(text),
            printable_ascii_symbols: count_printable_ascii_symbols(text),
            size_in_bytes_ascii: calc_size_data_in_ascii_bytes(text)
        };

        return Ok(info);
    }
}