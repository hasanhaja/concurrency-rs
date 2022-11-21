use core::{clear_dir, generate_inputs, Result, mkdir};
use rayon::prelude::{ParallelIterator, IntoParallelIterator};

fn initialize_computation_experiment() -> Result<()> {
    let input_path = "input-images";
    for path in vec![input_path, "seq-output-images", "mult-output-images", "async-output-images"].iter() {
        mkdir(path)?;
    }

    clear_dir(input_path)?;
    generate_inputs(25, input_path)?;

    Ok(())
}

fn initialize_io_experiment() -> Result<()> {
    let input_path = "input-images";

    let inputs = vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250];

    inputs.into_par_iter().for_each(|n| {
        let input = format!("{}-{}", input_path, n);
        mkdir(input.as_str()).unwrap();
        generate_inputs(n, input.as_str()).unwrap();
    });

    mkdir("mult-output-images")?;
    mkdir("async-output-images")?;

    Ok(())
}

fn main() -> Result<()> {
    // initialize_computation_experiment()
    initialize_io_experiment()
}
