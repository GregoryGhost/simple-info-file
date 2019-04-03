#![allow(dead_code)]


#[derive(Debug)]
pub struct InfoText {
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

pub fn get_info_text(text: &[&str]) -> std::io::Result<InfoText> {
    let info = InfoText {
        lines: count_line(text),
        printable_ascii_symbols: count_printable_ascii_symbols(text),
        size_in_bytes_ascii: calc_size_data_in_ascii_bytes(text)
    };

    return Ok(info);
}