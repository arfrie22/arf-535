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
    value: u32
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
        Self {
            value
        }
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


// #[derive(Debug)]
// pub struct WAVInput {
//     value: u32,
//     wav_file
// }

// impl InputStream for WAVInput {
//     fn get_next(&mut self) -> u32 {
//         self.value
//     }
// }

// impl Stream for ConstantInput {
//     fn set_clock_rate(&mut self, _clock_rate: usize) {}
// }

// #[derive(Debug)]
// pub struct WAVOutput {}

// impl OutputStream for NoOperationOutput {
//     fn set_next(&mut self, _value: u32) {}
// }

// impl Stream for NoOperationOutput {
//     fn set_clock_rate(&mut self, _clock_rate: usize) {}
// }