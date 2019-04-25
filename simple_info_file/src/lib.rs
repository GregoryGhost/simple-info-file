#[path = "simple_info_text.rs"] pub mod simple_info_text;
#[path = "file_helper.rs"] pub mod file_helper;


#[cfg(test)]
mod tests {
    use crate::simple_info_text::get_info_text;
    use crate::simple_info_text::InfoText;
    use crate::file_helper::read_file;

    const PATH_TO_TEST_FILE: &'static str = "test.txt";

    #[test]
    fn check_get_info_text() {
        let data = read_file(PATH_TO_TEST_FILE).unwrap();
        let actual = get_info_text(&data);
        let expected: std::io::Result<InfoText> = Ok(InfoText {
            lines: 3,
            printable_ascii_symbols: 14,
            size_in_bytes_ascii: 21
        });

        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}