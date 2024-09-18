use std::{env, fs::File, io::{BufReader, Read}};

use byteorder::{BigEndian, ReadBytesExt};

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
            self.read_image_file(arg);
        }
    }

    pub fn read_image_file(&mut self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);

        let mut address = reader
            .read_u16::<BigEndian>()
            .unwrap();
        while let Ok(instr) = reader.read_u16::<BigEndian>() {
            self.mem.write(address as usize, instr);
            address += 1;
        }
    }
}

pub fn set_up() {
    // C Code:
    //  signal(SIGINT, handle_interrupt);
    //  disable_input_buffering();
}
