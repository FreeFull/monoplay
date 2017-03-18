extern crate rodio;
extern crate nom;

mod moduledata;
mod replayer;

use moduledata::ModuleData;
use replayer::Replayer;

fn main() {
    let module = ModuleData::load("prelude.mon").unwrap();
    let replayer = Replayer::new(module);

    let endpoint = rodio::get_default_endpoint().expect("Unable to open audio device.");
    let sink = rodio::Sink::new(&endpoint);
    sink.append(replayer);
    sink.sleep_until_end();
}
