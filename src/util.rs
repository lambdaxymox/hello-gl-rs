use std::fs::File;
use std::io::Read;
use std::io;
use tga::TgaImage;
use std::os::raw;


pub fn file_contents(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = try!(File::open(filename));
    let mut buffer: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buffer));

    Ok(buffer)
}

pub fn read_tga(filename: &str) -> io::Result<(*const raw::c_void, i32, i32)> {
    let mut file = try!(File::open(filename));
    let tga_image = TgaImage::parse_from_file(&mut file).unwrap();
    let image = tga_image.pixels().collect::<Vec<[u8; 3]>>().as_ptr() as *const raw::c_void;
    let height = tga_image.height() as i32;
    let width = tga_image.width() as i32;

    Ok((image, height, width))
}

