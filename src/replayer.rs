use std::time::Duration;

use rodio;
use moduledata::ModuleData;

pub struct Replayer {
    data: ModuleData,
}

impl Replayer {
    pub fn new(data: ModuleData) -> Replayer {
        unimplemented!()
    }
}

impl Iterator for Replayer {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        unimplemented!()
    }
}

impl rodio::Source for Replayer {
    fn get_current_frame_len(&self) -> Option<usize> {
        None
    }

    fn get_channels(&self) -> u16 {
        1
    }

    fn get_samples_rate(&self) -> u32 {
        48000
    }

    fn get_total_duration(&self) -> Option<Duration> {
        None
    }
}
