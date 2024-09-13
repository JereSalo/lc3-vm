use std::env;

fn main() {
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

fn read_image(filename: &str) -> bool {
    // Open file, read, close file. To be Implemented.

    // Dummy
    println!("Reading image: {}", filename);
    true
}
