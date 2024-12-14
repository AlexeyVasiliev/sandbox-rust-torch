extern crate tch;
use tch::{Device, Cuda};

fn main() {
    // // Проверка наличия доступного CUDA-устройства
    // if Device::cuda_if_available()
    //     println!("CUDA доступен!");
    // } else {
    //     println!("CUDA недоступен.");
    // }

    // // Вывод информации о текущем устройстве
   //let device = Device::cuda_if_available();
   println!("Текущее устройство: {:?}", Device::cuda_if_available());
   println!("Текущее устройство: {:?}", Cuda::cudnn_is_available());
}