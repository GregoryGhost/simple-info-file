#[path = "simple_info_text.rs"] pub mod simple_info_text;
#[path = "file_helper.rs"] pub mod file_helper;

use simple_info_text::get_info_text;
use simple_info_text::InfoText;

use std::io;
use file_helper::read_file;

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
