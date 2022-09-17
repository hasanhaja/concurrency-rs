use std::{fs, io, path::PathBuf};

use image::{io::Reader, DynamicImage};
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

type Error = Box<dyn std::error::Error>;
type Result<T> = ::std::result::Result<T, Error>;

fn generate_inputs() -> Result<()> {
    let image = Reader::open("assets/flowers_original.jpg")?.decode()?;

    for n in 0..5 {
        image.save(format!("input-images/flowers-{}.jpg", n))?;
    }

    Ok(())
}

fn get_inputs() -> Result<Vec<PathBuf>> {
    let inputs = fs::read_dir("input-images")?;
    let inputs = inputs
        .map(|res| res.map(|e| e.path()))
        .filter(|res| res.is_ok() && res.as_ref().unwrap().is_file())
        .collect::<::std::result::Result<Vec<_>, io::Error>>()?;

    Ok(inputs)
}

fn process<F>(image_path: &PathBuf, destination: &str, tag: &str, f: F) -> Result<()>
where
    F: Fn(DynamicImage) -> DynamicImage,
{
    let image = Reader::open(image_path)?.decode()?;
    let filename = image_path.file_name().unwrap().to_str().unwrap();

    let image = f(image);

    image.save(format!("{}/{}-{}", destination, tag, filename))?;

    Ok(())
}

fn seq_process_images() -> Result<()> {
    let inputs = get_inputs()?;

    inputs.iter().for_each(|path| {
        process(path, "seq-output-images", "gray", |image| image.grayscale()).unwrap()
    });

    Ok(())
}

fn mult_process_images() -> Result<()> {
    let inputs = get_inputs()?;

    inputs.par_iter().for_each(|path| {
        process(path, "mult-output-images", "gray", |image| image.grayscale()).unwrap()
    });

    Ok(())
}

fn async_process_images() -> Result<()> {
    todo!("To be implemented");
}

fn main() -> Result<()> {
    // generate_inputs()
    // seq_process_images()
    mult_process_images()
}
