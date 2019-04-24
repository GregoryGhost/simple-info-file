#[path = "simple_info_text.rs"] pub mod simple_info_text;


#[cfg(test)]
mod tests {
    use crate::simple_info_text::get_info_text;
    use crate::simple_info_text::InfoText;

    const BUFFER: &'static [u8] = &[
        239u8, 187u8, 191u8, 49u8, 50u8, 51u8, 52u8, 13u8, 10u8,
        116u8, 101u8, 115u8, 116u8, 13u8, 10u8,
        33u8, 64u8, 35u8, 42u8, 38u8, 94u8];

    #[test]
    fn check_get_info_text() {
        let actual = get_info_text(&BUFFER.to_vec());
        let expected: std::io::Result<InfoText> = Ok(InfoText {
            lines: 3,
            printable_ascii_symbols: 14,
            size_in_bytes_ascii: 21
        });

        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}