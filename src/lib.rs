extern crate tch;
use tch::{Device, Cuda, Tensor};
pub fn cpu_gpu() {
    let gpu = Device::cuda_if_available();
    println!("Текущее устройство: {:?}", gpu);
    println!("Текущее CudaNN: {:?}", Cuda::cudnn_is_available());
    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    let tc = t.to_device(gpu);
    tc.print();
    t.print();
}