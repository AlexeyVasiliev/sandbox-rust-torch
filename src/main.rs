//use std::{fs::File, io::Read};

// This example illustrates how to use pre-trained vision models.
// model to get the imagenet label for some image.
//
// The pre-trained weight files containing the pre-trained weights can be found here:
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/resnet18.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/resnet34.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/densenet121.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/vgg13.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/vgg16.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/vgg19.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/squeezenet1_0.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/squeezenet1_1.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/alexnet.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/inception-v3.ot
// https://github.com/LaurentMazare/tch-rs/releases/download/mw/mobilenet-v2.ot
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/efficientnet-b0.safetensors
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/efficientnet-b1.safetensors
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/efficientnet-b2.safetensors
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/efficientnet-b3.safetensors
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/efficientnet-b4.safetensors
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/convmixer1536_20.ot
// https://github.com/LaurentMazare/ocaml-torch/releases/download/v0.1-unstable/convmixer1024_20.ot
// In order to obtain the dinov2 weights, e.g. dinov2_vits14.safetensors, run the
// src/vision/export_dinov2.py
// use anyhow::{bail, Result};

// pub fn main() -> Result<()> {
//     let args: Vec<_> = std::env::args().collect();
//     let (weights, image) = match args.as_slice() {
//         [_, w, i] => (std::path::Path::new(w), i.to_owned()),
//         _ => bail!("usage: main resnet18.ot image.jpg"),
//     };
//     let mut file = File::open(image)?;
//     let mut data = vec![];
//     file.read_to_end(&mut data)?;

//     let output = sandbox_rust_torch::recognize_image(weights, data)?;
//     println!("{output:?}");
//     Ok(())
// }


use std::path;

use futures::stream::StreamExt;
//use anyhow::bail;
#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("localhost:4222").await?;
    let mut subscriber = client.subscribe("messages").await?;
    let weights =path::Path::new("resnet34.ot");
    while let Some(message) = subscriber.next().await {
        let bytes = message.payload;
        let output = sandbox_rust_torch::recognize_image(weights, bytes.to_vec())?;
        println!("{output:?}");
    }
    Ok(())
}
