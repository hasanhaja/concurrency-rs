use core::{generate_inputs, mult_process_images, Result, clear_outputs, async_process_images};
use tokio;

#[tokio::main]
async fn main() {
    // generate_inputs(15)
    // seq_process_images()
    // mult_process_images()

    let blocking_task = tokio::task::spawn_blocking(|| {
        async_process_images(&0.5).unwrap();
    });

    blocking_task.await.unwrap(); 
    // clear_outputs("seq-output-images")

    // clear_outputs("input-images")
}
