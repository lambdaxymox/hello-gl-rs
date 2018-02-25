use std::fs::File;
use std::io::BufReader;
use std::io::Read;


fn file_contents(filename: &str, length: &mut isize) -> Option<(Vec<u8>, usize)> {
    let mut file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    let length = buf_reader.read_to_end(&mut buffer).unwrap();

    Some((buffer, length))
}
