use std::{fs, io, path::PathBuf};

use image::{io::Reader, DynamicImage};
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = ::std::result::Result<T, Error>;

pub fn generate_inputs(inputs: i32) -> Result<()> {
    let image = Reader::open("assets/flowers_original.jpg")?.decode()?;

    (0..inputs).into_par_iter().for_each(|n| {
        image
            .save(format!("input-images/flowers-{}.jpg", n))
            .unwrap();
    });

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

#[inline]
pub fn clear_outputs(output_path: &str) -> Result<()> {
    let outputs = fs::read_dir(output_path)?;

    for res in outputs {
        fs::remove_file(res?.path())?
    }

    Ok(())
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

#[inline]
pub fn seq_process_images(blur_sigma: f32) -> Result<()> {
    let inputs = get_inputs()?;

    inputs.iter().for_each(|path| {
        process(path, "seq-output-images", "blur", |image| image.blur(blur_sigma)).unwrap()
    });

    Ok(())
}

#[inline]
pub fn mult_process_images(blur_sigma: f32) -> Result<()> {
    let inputs = get_inputs()?;

    inputs.par_iter().for_each(|path| {
        process(path, "mult-output-images", "blur", |image| {
            image.blur(blur_sigma)
        })
        .unwrap()
    });

    Ok(())
}

pub fn async_process_images() -> Result<()> {
    todo!("To be implemented");
}
