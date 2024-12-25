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

use std::path;

use async_nats;
use futures::stream::StreamExt;
pub mod cfg;
use cfg::Config;
use sandbox_rust_torch::ImageDefenition;
use serde::{Serialize,Deserialize};
use serde_json::{to_string, from_str};
#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    Config::init_env();
    let config = cfg::Config::build()?;
    let client = async_nats::connect(config.nats_conn_string).await?;
    let mut subscriber = client.subscribe(config.nats_subject_in).await?;
    let nats_subject_out = config.nats_subject_out;
    let weights =path::Path::new("resnet34.ot");
    println!("ready to recognize an image from nats");
    while let Some(message) = subscriber.next().await {
        let image_message_str = std::str::from_utf8(&message.payload)?;
        let deserialized_image_message: ImageRequest = from_str(&image_message_str)?;
        println!("image id {}",deserialized_image_message.id);
        let output = sandbox_rust_torch::recognize_image(weights, deserialized_image_message.image.to_vec())?;
       // let s = format!("{output:?}");
        let response = ImageResponse {
            id: deserialized_image_message.id,
            description: output
        };
        let json = to_string(&response)?;
        let s = json.clone();
        client.publish(nats_subject_out.clone(), json.into()).await?;
        println!("publish result: {s}");
    }
    Ok(())
}


#[derive(Deserialize)]
struct ImageRequest {
    id: String,
    image: Vec<u8>,
}

#[derive(Serialize)]
struct ImageResponse {
    id: String,
    description: ImageDefenition,
}
