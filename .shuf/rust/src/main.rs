use rand::seq::IteratorRandom;
use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();
    let mut rng = rand::thread_rng();
    let output = paths.choose(&mut rng).unwrap().unwrap().file_name().into_string().unwrap();
    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|i| i == "mpv") {
        std::process::Command::new("mpv")
            .arg(output)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .output()
            .expect("Could not play the the image, video, or directory in mpv.");
    } else {
        println!("{}", output);
    }
}
