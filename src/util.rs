use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io;
use tga::TgaImage;


pub fn file_contents(filename: &str) -> io::Result<(Vec<u8>, usize)> {
    let file = try!(File::open(filename));
    let mut buf_reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    let length = try!(buf_reader.read_to_end(&mut buffer));

    Ok((buffer, length))
}

pub fn read_tga(filename: &str) -> io::Result<(Vec<[u8; 3]>, usize, usize)> {
    let mut file = try!(File::open(filename));
    let tga_image = TgaImage::parse_from_file(&mut file).unwrap();
    let image = tga_image.pixels().collect::<Vec<[u8; 3]>>();
    let height = tga_image.height();
    let width = tga_image.width();

    Ok((image, height, width))
}

