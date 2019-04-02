#[path = "simple_info_file.rs"] mod simple_info_file;

#[cfg(test)]
mod tests {
    use crate::simple_info_file::simple_info_file::count_line;
    use crate::simple_info_file::simple_info_file::count_printable_ascii_symbols;
    use crate::simple_info_file::simple_info_file::calc_size_data_in_ascii_bytes;

    const COMMON_DATA: &'static [&'static str] = &["1235", "123", "test\x7f"];

    #[test]
    fn check_calc_count_line() {
        let actual = count_line(COMMON_DATA);
        let expected = 3;
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_calc_size_data_in_bytes_ascii() {
        let actual = calc_size_data_in_ascii_bytes(COMMON_DATA);
        let expected = 12;

        assert_eq!(actual, expected);
    }

    #[test]
    fn check_calc_printable_ascii_symbols() {
        let actual = count_printable_ascii_symbols(COMMON_DATA);
        let expected = 11;

        assert_eq!(actual, expected);
    }

    #[test]
    fn check_get_info_file() {
        panic!("todo");
    }
}