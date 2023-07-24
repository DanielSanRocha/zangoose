pub fn denoise(buffer: Vec<f32>) -> Vec<f32> {
    let mut data = Vec::<f32>::new();

    for x in buffer.iter() {
        data.push(*x);
    }

    data
}