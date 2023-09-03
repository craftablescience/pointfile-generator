use image::io::Reader as ImageReader;

const FRAMES_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/frames");

fn main() -> Result<(), image::ImageError> {
    let mut image_count = 0;
    let paths = std::fs::read_dir(FRAMES_DIR).unwrap();
    for _ in paths {
        image_count += 1;
    }

    for i in 0..image_count {
        println!("hello {i}");

        let _image = ImageReader::open(format!("{FRAMES_DIR}/{i:04}.png"))?.decode()?.as_bytes();
    }

    Ok(())
}
