use anyhow::{ Result,Error};
use tch::nn::FuncT;
use tch::vision::imagenet;

pub fn recognize_image(model: &mut FuncT<'static>, data:  Vec<u8>) -> Result<tch::Tensor,Error>{
	let image = imagenet::load_image_and_resize224_from_memory(&data)?
		.to_device(tch::Device::cuda_if_available());

	// Apply the forward pass of the model to get the logits
	let output = image
		.unsqueeze(0)
		.apply_t(model, false)
		.softmax(-1, tch::Kind::Float);
    Ok(output)
}
