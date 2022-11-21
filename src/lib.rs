use image::load_from_memory;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::{fs, io, path::PathBuf, path::Path};
use tokio::fs::{read, write};

use image::{io::Reader, DynamicImage, ImageOutputFormat};
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = ::std::result::Result<T, Error>;

pub fn generate_inputs(inputs: i32, path: &str) -> Result<()> {
    let image = Reader::open("assets/flowers_original.jpg")?.decode()?;

    (0..inputs).into_par_iter().for_each(|n| {
        image
            .save(format!("{}/flowers-{}.jpg", path, n))
            .unwrap();
    });

    Ok(())
}

pub fn mkdir(path: &str) -> Result<()> {
    if !Path::new(path).is_dir() {
        fs::create_dir(path)?;
    }

    Ok(())
}

fn get_inputs(path: &str) -> Result<Vec<PathBuf>> {
    let inputs = fs::read_dir(path)?;
    let inputs = inputs
        .map(|res| res.map(|e| e.path()))
        .filter(|res| res.is_ok() && res.as_ref().unwrap().is_file())
        .collect::<::std::result::Result<Vec<_>, io::Error>>()?;

    Ok(inputs)
}

#[inline]
pub fn clear_dir(path: &str) -> Result<()> {
    let outputs = fs::read_dir(path)?;

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

async fn async_process<F>(image_path: PathBuf, destination: &str, tag: &str, f: F) -> Result<()>
where
    F: Fn(DynamicImage) -> DynamicImage,
{
    let image_buffer = read(&image_path).await?;
    let image = load_from_memory(&image_buffer).unwrap();
    let filename = image_path.file_name().unwrap().to_str().unwrap();

    let image = f(image);

    let buffer = to_buffer(image);

    write(format!("{}/{}-{}", destination, tag, filename), buffer).await?;

    Ok(())
}

// source: https://github.com/peerigon/wasm-image/blob/master/rust-image-wrapper/src/lib.rs
fn to_buffer(image: DynamicImage) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());

    image
        .write_to(&mut cursor, ImageOutputFormat::Jpeg(80))
        .unwrap();

    cursor.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut buffer = Vec::new();
    cursor.read_to_end(&mut buffer).unwrap();

    buffer
}

#[inline]
pub fn seq_process_images(input_path: &str, output_path: &str, blur_sigma: f32) -> Result<()> {
    let inputs = get_inputs(input_path)?;

    inputs.iter().for_each(|path| {
        process(path, output_path, "blur", |image| {
            image.blur(blur_sigma)
        })
        .unwrap()
    });

    Ok(())
}

#[inline]
pub fn mult_process_images(input_path: &str, output_path: &str, blur_sigma: f32) -> Result<()> {
    let inputs = get_inputs(input_path)?;

    inputs.par_iter().for_each(|path| {
        process(path, output_path, "blur", |image| {
            image.blur(blur_sigma)
        })
        .unwrap()
    });

    Ok(())
}

// https://stackoverflow.com/questions/63434977/how-can-i-spawn-asynchronous-methods-in-a-loop
#[inline]
pub async fn async_process_images(input_path: &str, output_path: &'static str, blur_sigma: &'static f32) -> Result<()> {
    let inputs = get_inputs(input_path)?;

    let tasks: Vec<_> = inputs
        .into_iter()
        .map(|path| {
            tokio::spawn(async move {
                async_process(path, output_path, "blur", |image| {
                    image.blur(*blur_sigma)
                })
                .await
                .unwrap()
            })
        })
        .collect();

    for task in tasks {
        task.await?;
    }

    Ok(())
}
