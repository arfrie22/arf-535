use crate::raw_cast_from_i32;

pub trait Stream: std::fmt::Debug {
    fn set_clock_rate(&mut self, clock_rate: usize);
}

pub trait InputStream: Stream {
    fn get_next(&mut self) -> u32;
}

pub trait OutputStream: Stream {
    fn set_next(&mut self, value: u32);
}

#[derive(Debug)]
pub struct ConstantInput {
    value: u32,
}

impl InputStream for ConstantInput {
    fn get_next(&mut self) -> u32 {
        self.value
    }
}

impl Stream for ConstantInput {
    fn set_clock_rate(&mut self, _clock_rate: usize) {}
}

impl ConstantInput {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct NoOperationOutput {}

impl OutputStream for NoOperationOutput {
    fn set_next(&mut self, _value: u32) {}
}

impl Stream for NoOperationOutput {
    fn set_clock_rate(&mut self, _clock_rate: usize) {}
}

impl NoOperationOutput {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct WavInput {
    sample_rate: u32,
    wav_data: Vec<u32>,
    wav_pointer: f64,
    clock_rate: usize,
}

impl WavInput {
    pub fn new(wav_file: &str) -> Self {
    let reader = hound::WavReader::open(wav_file).unwrap();
    let wav_spec = reader.spec();
    let channels = wav_spec.channels as usize;
    let wav_data = reader.into_samples::<i32>().enumerate().filter(|(i, _)| i % channels == 0).map(|s| raw_cast_from_i32(s.1.unwrap_or(0))).collect();
        Self {
            sample_rate: wav_spec.sample_rate,
            wav_data,
            wav_pointer: 0.0,
            clock_rate: 0,
        }
    }
}

impl InputStream for WavInput {
    fn get_next(&mut self) -> u32 {
        let ptr = self.wav_pointer as usize;
        if ptr >= self.wav_data.len() {
            0
        } else {
            if self.clock_rate > 0 {
                self.wav_pointer += (self.sample_rate as f64) / (self.clock_rate as f64);
            }
            self.wav_data[ptr]
        }
    }
}

impl Stream for WavInput {
    fn set_clock_rate(&mut self, clock_rate: usize) {
        self.clock_rate = clock_rate;
    }
}

// #[derive(Debug)]
// pub struct WAVOutput {}

// impl OutputStream for NoOperationOutput {
//     fn set_next(&mut self, _value: u32) {}
// }

// impl Stream for NoOperationOutput {
//     fn set_clock_rate(&mut self, _clock_rate: usize) {}
// }
