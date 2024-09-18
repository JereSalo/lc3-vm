use std::{env, fs::File};

use crate::hardware::vm::VM;

impl VM {
    pub fn load_arguments(&mut self) {
        let args: Vec<String> = env::args().collect();
    
        if args.len() < 2 {
            eprintln!("lc3 [image-file1] ...");
            return;
        }
    
        // Iterate over each argument (skipping the first one which is the program name)
        for arg in &args[1..] {
            if !self.read_image_file(arg) {
                // Show error message and exit with status code 1
                eprintln!("failed to load image: {}", arg);
                return;
            }
        }
    }

    fn read_image_file(&self, filename: &str) -> bool {
        let file = File::open(filename).expect("Error opening file");

        

    }
}


fn swap16(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

pub fn set_up() {
    // C Code:
    //  signal(SIGINT, handle_interrupt);
    //  disable_input_buffering();
}
