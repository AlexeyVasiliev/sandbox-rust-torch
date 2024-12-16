extern crate tch;
use tch::{Device, Cuda, Tensor};

fn main() {
    let gpu = Device::cuda_if_available();
    let cpu = Device::Cpu;
    println!("Текущее устройство: {:?}", gpu);
    println!("Текущее CudaNN: {:?}", Cuda::cudnn_is_available());
    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    let tc = t.to_device(gpu);
    let cc = t.to_device(cpu);
    tc.print();
    cc.print();
}
  