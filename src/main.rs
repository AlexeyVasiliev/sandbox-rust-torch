use std::{fs::File, io::Read};

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
use anyhow::{bail, Result};

pub fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let (weights, image) = match args.as_slice() {
        [_, w, i] => (std::path::Path::new(w), i.to_owned()),
        _ => bail!("usage: main resnet18.ot image.jpg"),
    };
    let mut file = File::open(image)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let output = sandbox_rust_torch::recognize_image(weights, data)?;
    println!("{output:?}");
    Ok(())
}


// use std::{fs::File, io::Write};

// use bytes::Bytes;
// use futures::stream::StreamExt;
// use show_image::{ImageInfo, ImageView};
// #[tokio::main]
// async fn main() -> Result<(), async_nats::Error> {
//     // Connect to the NATS server
//     let client = async_nats::connect("localhost:4222").await?;

//     // // Subscribe to the "messages" subject
//     let mut subscriber = client.subscribe("messages").await?;

//     // // Publish messages to the "messages" subject
//     // for _ in 0..10 {
//     //     client.publish("messages", "dataA".into()).await?;
//     // }

//     // Receive and process messages
//     let mut i = 0;
//     while let Some(message) = subscriber.next().await {
//         let bytes:Bytes = message.payload;
//         let mut f = File::create(format!("foo_{i}.jpg"))?;
//         let _ = f.write_all(&bytes);
//         i = i + 1;
//        // f.sync_data()?;
//     }

//     Ok(())
// }
