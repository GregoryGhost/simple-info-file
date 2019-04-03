#[path = "simple_info_text.rs"] pub mod simple_info_text;


#[cfg(test)]
mod tests {
    use crate::simple_info_text::count_line;
    use crate::simple_info_text::count_printable_ascii_symbols;
    use crate::simple_info_text::calc_size_data_in_ascii_bytes;
    use crate::simple_info_text::get_info_text;
    use crate::simple_info_text::InfoText;

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
    fn check_get_info_text() {
        let actual = get_info_text(COMMON_DATA);
        let expected: std::io::Result<InfoText> = Ok(InfoText {
            lines: 3,
            printable_ascii_symbols: 11,
            size_in_bytes_ascii: 12
        });

        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}