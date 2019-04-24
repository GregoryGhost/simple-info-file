extern crate simple_info_text_lib;
use simple_info_text_lib::simple_info_text::get_info_text;
use simple_info_text_lib::simple_info_text::InfoText;

use std::io;
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path).expect("Файл не найден");

    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Не удалось прочитать файл");

    Ok(data)
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    println!("Введите путь до файла:");
    io::stdin().read_line(&mut input)?;
    let path = input.trim();

    let info: io::Result<InfoText> = match read_file(path) {
        Ok(buffer) => get_info_text(&buffer),
        Err(e) => return Err(e),
    };

    println!("Информация: {:?}", info);

    Ok(())
}
