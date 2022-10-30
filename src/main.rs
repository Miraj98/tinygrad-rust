mod mnist;
mod tinygrad;

use mnist::MnistTrainData;
use ndarray::{Array3};
use tinygrad::nn::neural_net;
use image::{GrayImage, imageops};

fn main() {
    let mnist_data = MnistTrainData::load();
    let idx = 60000;
    // let mut expected_out = Array3::<f64>::zeros((60000, 10, 1));
    // let in_vec = mnist_data.get_image_nn_input(idx);
    // for i in 0..mnist_data.raw_labels_data.len() - 8 {
    //     expected_out[(i, mnist_data.get_image_label(i) as usize, 0)] = 1.;
    // }
    // let out_vec = mnist_data.get_image_label_vector(idx);
    // let mut model = neural_net::Model::init(28 * 28, 10, vec![2, 2]);
    // model.train(&in_vec, &out_vec, 0.1);
     
    let in_buf = mnist_data.get_img_buffer(idx);
    println!("{:?}", in_buf);
    // image::save_buffer_with_format("1.png", in_buf, 28, 28, image::ColorType::L8, image::ImageFormat::Png).unwrap();
    let something = GrayImage::from_raw(28, 28, in_buf.to_vec()).unwrap();

    something.save_with_format("2.png", image::ImageFormat::Png).unwrap();
}