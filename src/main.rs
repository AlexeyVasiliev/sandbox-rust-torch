use async_nats;
use futures::stream::StreamExt;
pub mod cfg;
use cfg::Config;
use sandbox_rust_torch::ImageDefenition;
use serde::{Serialize,Deserialize};
use serde_json::{to_string, from_str};
use anyhow::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    Config::init_env();
    let config = cfg::Config::build()?;
    let client = async_nats::connect(config.nats_conn_string).await?;
    let mut subscriber = client.subscribe(config.nats_subject_in).await?;
    let nats_subject_out = config.nats_subject_out;
    let mut vs =  tch::nn::VarStore::new(tch::Device::cuda_if_available());
	println!("Cuda support: {}",vs.device().is_cuda());
	let model = tch::vision::resnet::resnet34(&vs.root(), 1000);
    vs.load("resnet34.safetensors")?;
    println!("ready to recognize an image from nats");
    while let Some(message) = subscriber.next().await {
        let image_message_str = std::str::from_utf8(&message.payload)?;
        let deserialized_image_message: ImageRequest = from_str(&image_message_str)?;
        println!("image id {}",deserialized_image_message.id);
        let output = sandbox_rust_torch::recognize_image(&model, deserialized_image_message.image.to_vec());
        match output {
            Ok(out) => {
                let response = ImageResponse {
                    id: deserialized_image_message.id,
                    description: out
                };
                let json = to_string(&response)?;
                let s = json.clone();
                client.publish(nats_subject_out.clone(), json.into()).await?;
                println!("publish result: {s}");
            },
            Err(err) => println!("{}",err)
        }
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
