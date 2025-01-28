use anyhow::Error;
use serde::Serialize;
use tch::{nn::FuncT, vision::imagenet};


mod nn;
#[derive(Debug,Serialize)]
pub struct ImageClass {
    pub class:String,
    pub probability:f64
}
#[derive(Debug,Serialize)]
pub struct ImageDefenition {
    pub classes:  Vec<ImageClass>
}


pub fn recognize_image(model: &mut FuncT<'static>, data: Vec<u8>) -> Result<ImageDefenition,Error>{
    let output = nn::recognize_image(model,data)?;
    let mut image_definition = ImageDefenition {
        classes: Vec::new()
    };
    for (probability, class) in imagenet::top(&output, 5).iter() {
        image_definition.classes.push(
            ImageClass {
                class: class.to_string(),
                probability: *probability
            }
        );
    }
    Ok(image_definition)
}