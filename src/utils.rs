use std::env;

pub fn load_arguments() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("lc3 [image-file1] ...");
        return;
    }

    // Iterate over each argument (skipping the first one which is the program name)
    for arg in &args[1..] {
        if !read_image(arg) {
            // Show error message and exit with status code 1
            eprintln!("failed to load image: {}", arg);
            return;
        }
    }
}

fn swap16(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

fn read_image(filename: &str) -> bool {
    // Open file, read, close file. To be Implemented.

    // Dummy
    println!("Reading image: {}", filename);
    true
}

pub fn set_up() {
    // C Code:
    //  signal(SIGINT, handle_interrupt);
    //  disable_input_buffering();
}
