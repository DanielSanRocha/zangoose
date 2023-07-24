use std::cmp;
use libm;

pub fn denoise(last_buffer: Vec<f32>, buffer: Vec<f32>, frac: f32) -> Vec<f32> {
    let mut output = vec![0.0;buffer.len()];
    let window = ((frac.powi(2) * buffer.len() as f32)/2.0) as usize;

    if last_buffer.len() == 0 {
        return output
    }

    for i in 0..buffer.len() {
        let min = cmp::max(0,i as i32 - window as i32) as usize;

        if min > 0 {
            let max = cmp::min(buffer.len()-1,i + window) as usize;
            output[i] = buffer[min..max].iter().sum::<f32>()/((max - min + 1) as f32);
        } else {
            let max = cmp::min(buffer.len()-1,i + window) as usize;
            let x = buffer[0..max].iter().sum::<f32>();
            let y = last_buffer[(last_buffer.len() - window + i - 1)..last_buffer.len()-1].iter().sum::<f32>();
            output[i] = (x+y)/((max + window - i + 1) as f32);
        }
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