use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path).expect("Файл не найден");

    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Не удалось прочитать файл");

    Ok(data)
}