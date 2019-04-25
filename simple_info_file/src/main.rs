#[path = "simple_info_text.rs"] pub mod simple_info_text;
#[path = "file_helper.rs"] pub mod file_helper;

use simple_info_text::get_info_text;
use simple_info_text::InfoText;

use std::io;
use file_helper::read_file;
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    let path_arg = env::args().nth(1).expect("Укажите путь к файлу");
    let path = Path::new(&path_arg);

    let file = read_file(&path).unwrap();
    let info: io::Result<InfoText> = get_info_text(&file);

    println!("Информация: {:?}", info);

    Ok(())
}
