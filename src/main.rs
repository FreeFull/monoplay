extern crate rodio;
extern crate nom;

mod moduledata;
mod replayer;
mod constants;

use moduledata::ModuleData;
use replayer::Replayer;

use std::env;

fn main() {
    let path = env::args().nth(1).expect("Please provide the filename as an argument.");
    let module = ModuleData::load(&path).unwrap();
    let replayer = Replayer::new(module);

    let endpoint = rodio::get_default_endpoint().expect("Unable to open audio device.");
    let sink = rodio::Sink::new(&endpoint);
    sink.append(replayer);
    sink.sleep_until_end();
}
