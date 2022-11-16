use std::{fs, io, path::PathBuf};
use std::io::{Cursor, Read, Seek, SeekFrom};
use image::load_from_memory;
use tokio::fs::{read, write};

use image::{io::Reader, DynamicImage, ImageOutputFormat};
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

// https://stackoverflow.com/questions/63434977/how-can-i-spawn-asynchronous-methods-in-a-loop
#[inline]
pub fn async_process_images(blur_sigma: &'static f32) -> Result<()> {
    let inputs = get_inputs()?;
    println!("Here!");
    inputs
        .into_iter()
        .for_each(|path| {
            tokio::spawn(async move {
                async_process(path, "async-output-images", "blur", |image| {
                    image.blur(*blur_sigma)
                }).await.unwrap();
            });
        });

    Ok(())
}
