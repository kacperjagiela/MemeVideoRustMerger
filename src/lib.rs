use std::fs;

fn load_file(path: &str) {
    let file = fs::read(path);

    match file {
        Ok(data) => {
            println!("File loaded: {:?}", data);
        }
        Err(_) => {
            println!("Error loading file");
        }
    }
}

pub fn run() -> Result<(), ()> {
    println!("test");

    load_file("rec.mov");

    Ok(())
}
