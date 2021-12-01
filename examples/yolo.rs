use std::collections::HashMap;
// Indicates a f32 overflow in an intermediate Collatz value
// use wasm_bindgen_test::*;

use std::time::Instant;
// Args Management
async fn run() {
    let steps = execute_gpu().await;
    println!("steps: {:#?}", steps);
    // println!("steps[1..5]: {:#?}", &steps[0..5]);
    #[cfg(target_arch = "wasm32")]
    // log::info!("steps[0..5]: {:#?}", &steps[0..5]);
    assert_eq!(steps[0..5], [0.0, 0.0, 0.0, 0.0, 0.0]);
}

// Hardware management
async fn execute_gpu() -> Vec<f32> {
    let n: usize = 224;
    let mut input_data = HashMap::new();

    let data: Vec<f32> = (0..n * n).map(|x| x as f32).collect();
    input_data.insert("data".to_string(), data.as_slice());

    let mut session = wonnx::Session::from_path("examples/data/models/vgg19-7.onnx")
        .await
        .unwrap();
    let time_pre_compute = Instant::now();
    let a = wonnx::run(&mut session, input_data).await.unwrap();
    let time_post_compute = Instant::now();
    println!(
        "time: post_compute: {:#?}",
        time_post_compute - time_pre_compute
    );
    a
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        //  console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}