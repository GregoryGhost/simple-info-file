#[path = "lib1.rs"] mod simple_info_file;

#[cfg(test)]
mod tests {
    use crate::simple_info_file::simple_info_file::count_line;

    const COMMON_DATA: &'static [&'static str] = &["1235", "123", "test\x7f"];

    #[test]
    fn check_calc_count_line() {
        let actual = count_line(COMMON_DATA);
        let expected = 3;
        
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_calc_size_data_in_bytes_ascii() {
        panic!("todo");
    }

    #[test]
    fn check_calc_ascii_symbols() {
        panic!("todo");
    }

    #[test]
    fn check_get_info_file() {
        panic!("todo");
    }
}