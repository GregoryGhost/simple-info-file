extern crate simple_info_text_lib;
use simple_info_text_lib::simple_info_text::get_info_text;

use std::io;
use std::io::BufRead;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut input = String::new();


    println!("Введите путь до файла:");
    io::stdin().read_line(&mut input)?;
    let path = input.trim();

    let mut file = File::open(path).expect("Файл не найден");
    let data: Vec<String> = io::BufReader::new(file).lines().map(|line| line.unwrap()).collect();
    let data: Vec<&str> = data.iter().map(|line| line as &str).collect();
    let data: &[&str] = &data;

    let info = get_info_text(data);

    println!("Информация о файле: {:?}", info);

    Ok(())
}
