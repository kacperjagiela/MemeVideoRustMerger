use std::process;

use left_and_right_ear_sound_merger::run;

fn main() {
    if let Err(_) = run() {
        eprintln!("Application error");
        process::exit(1);
    }
}
