use std::cmp;
use libm;

pub fn denoise(last_buffer: Vec<f32>, buffer: Vec<f32>, frac: f32) -> Vec<f32> {
    let mut output = vec![0.0;buffer.len()];
    let mut window = ((frac.powi(2) * buffer.len() as f32)/2.0) as usize;

    if last_buffer.len() == 0 {
        return output
    }

    if window > buffer.len() {
        window = buffer.len();
    }

    for i in 0..window {
        let min = cmp::max(0, last_buffer.len() - 2 * window + i) as usize;
        let max = cmp::min(buffer.len(), i);

        let x = last_buffer[min..last_buffer.len() - 1].iter().sum::<f32>();
        let y = buffer[0..max].iter().sum::<f32>();

        output[i] = (x+y)/((2*window + 1) as f32);
    }

    for i in window..buffer.len() {
        let min = i - window;
        let max = cmp::min(buffer.len(), i + window);

        let x = buffer[min..max].iter().sum::<f32>();
        output[i] = x/((2*window + 1) as f32);
    }

    output
}

pub fn drive(buffer: Vec<f32>, factor: f32) -> Vec<f32> {
    let mut data = vec![0.0; buffer.len()];

    for (i, x) in buffer.iter().enumerate() {
        data[i] = libm::tanh((factor * *x) as f64) as f32;
    }

    data
}