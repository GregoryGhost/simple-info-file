#![allow(dead_code)]


#[derive(Debug)]
pub struct InfoText {
    pub lines: usize,
    pub printable_ascii_symbols: usize,
    pub size_in_bytes_ascii: usize,
}


fn count_ascii_chars(data: &Vec<u8>, _callback: fn(u8) -> bool) -> usize {
    data.iter().fold(0, |acc, i| match _callback(*i) {
        true => acc + 1,
        false => acc,
    })
}

fn count_line(data: &Vec<u8>) -> usize {
    count_ascii_chars(data, |i| i == 10) + 1
}

fn count_printable_ascii_symbols(data: &Vec<u8>) -> usize {
    count_ascii_chars(data, |i| 32u8 <= i && 126u8 <= i)
}

fn calc_size_data_in_ascii_bytes(data: &Vec<u8>) -> usize {
    data.len()
}

pub fn get_info_text(text: &Vec<u8>) -> std::io::Result<InfoText> {
    let info = InfoText {
        lines: count_line(text),
        printable_ascii_symbols: count_printable_ascii_symbols(text),
        size_in_bytes_ascii: calc_size_data_in_ascii_bytes(text)
    };

    return Ok(info);
}