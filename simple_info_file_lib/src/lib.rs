#[path = "simple_info_file.rs"] mod simple_info_file;

#[cfg(test)]
mod tests {
    use crate::simple_info_file::simple_info_file::count_line;
    use crate::simple_info_file::simple_info_file::count_printable_ascii_symbols;
    use crate::simple_info_file::simple_info_file::calc_size_data_in_ascii_bytes;
    use crate::simple_info_file::simple_info_file::get_info_file;
    use crate::simple_info_file::simple_info_file::InfoFile;

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

    use std::env;
    use std::path::Path;

    #[test]
    fn check_get_info_file() {
        let path = env::current_dir()?.display().join("test.txt")?.to_str();
        let actual = get_info_file(path);
        let expected: std::io::Result<InfoFile> = Ok(InfoFile {
            lines: 3,
            printable_ascii_symbols: 14,
            size_in_bytes_ascii: 14
        });

        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}