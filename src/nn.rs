use anyhow::{bail, Context, Result,Error};
use tch::nn::ModuleT;
use tch::vision::{
    alexnet, convmixer, densenet, dinov2, efficientnet, imagenet, inception, mobilenet, resnet,
    squeezenet, vgg,
};

pub fn recognize_image(weights:&std::path::Path, image: String) -> Result<tch::Tensor,Error>{
    let image = imagenet::load_image_and_resize224(image)?;

    // Create the model and load the weights from the file.
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let net: Box<dyn ModuleT> =
        match weights.file_stem().context("no stem")?.to_str().context("invalid stem")? {
            "resnet18" => Box::new(resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT)),
            "resnet34" => Box::new(resnet::resnet34(&vs.root(), imagenet::CLASS_COUNT)),
            "densenet121" => Box::new(densenet::densenet121(&vs.root(), imagenet::CLASS_COUNT)),
            "vgg13" => Box::new(vgg::vgg13(&vs.root(), imagenet::CLASS_COUNT)),
            "vgg16" => Box::new(vgg::vgg16(&vs.root(), imagenet::CLASS_COUNT)),
            "vgg19" => Box::new(vgg::vgg19(&vs.root(), imagenet::CLASS_COUNT)),
            "squeezenet1_0" => Box::new(squeezenet::v1_0(&vs.root(), imagenet::CLASS_COUNT)),
            "squeezenet1_1" => Box::new(squeezenet::v1_1(&vs.root(), imagenet::CLASS_COUNT)),
            "alexnet" => Box::new(alexnet::alexnet(&vs.root(), imagenet::CLASS_COUNT)),
            "inception-v3" => Box::new(inception::v3(&vs.root(), imagenet::CLASS_COUNT)),
            "mobilenet-v2" => Box::new(mobilenet::v2(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b0" => Box::new(efficientnet::b0(&vs.root(), imagenet::CLASS_COUNT)),
            // Maybe the higher resolution models should be handled differently.
            "efficientnet-b1" => Box::new(efficientnet::b1(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b2" => Box::new(efficientnet::b2(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b3" => Box::new(efficientnet::b3(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b4" => Box::new(efficientnet::b4(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b5" => Box::new(efficientnet::b5(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b6" => Box::new(efficientnet::b6(&vs.root(), imagenet::CLASS_COUNT)),
            "efficientnet-b7" => Box::new(efficientnet::b7(&vs.root(), imagenet::CLASS_COUNT)),
            "convmixer1536_20" => Box::new(convmixer::c1536_20(&vs.root(), imagenet::CLASS_COUNT)),
            "convmixer1024_20" => Box::new(convmixer::c1024_20(&vs.root(), imagenet::CLASS_COUNT)),
            "dinov2_vits14" => Box::new(dinov2::vit_small(&vs.root())),
            s => bail!("unknown model for {s}, use a weight file named e.g. resnet18.ot"),
        };
    vs.load(weights)?;

    // Apply the forward pass of the model to get the logits.
    let output =
        net.forward_t(&image.unsqueeze(0), /* train= */ false).softmax(-1, tch::Kind::Float);
    
    // let mut result:String = "".to_string();
    // for (probability, class) in imagenet::top(&output, 5).iter() {
    //     result.push_str(format!("{:50} {:5.2}%", class, 100.0 * probability).as_str());
    // }
    Ok(output)
}
