use std::error::Error;

use image::io::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let image = Reader::open("input-images/flowers_original.jpg")?.decode()?;

    let image = image.grayscale();

    image.save("output-images/flowers_original.jpg")?;

    Ok(())
}
