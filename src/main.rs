extern crate rodio;

use std::path::Path;
use std::time::Duration;

struct ModuleData {
}

impl ModuleData {
    fn load<P: AsRef<Path>>(file: P) -> ModuleData {
        let file = file.as_ref();
        unimplemented!()
    }
}

struct Replayer {
    data: ModuleData,
}

impl Replayer {
    fn new(data: ModuleData) -> Replayer {
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

fn main() {
    let module = ModuleData::load("prelude.mon");
    let replayer = Replayer::new(module);

    let endpoint = rodio::get_default_endpoint().expect("Unable to open audio device.");
    let sink = rodio::Sink::new(&endpoint);
    sink.append(replayer);
    sink.sleep_until_end();
}
