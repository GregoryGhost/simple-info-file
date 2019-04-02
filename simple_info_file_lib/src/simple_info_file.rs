pub mod simple_info_file {
    fn count_ascii_chars(data: &[& str], _callback: fn(char) -> bool) -> usize {
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

    fn calc_size_data_in_ascii_bytes(data: &str) -> i32 {
        panic!("todo");
    }

    fn get_info_file(data: &str) -> i32 {
        panic!("todo");
    }
}