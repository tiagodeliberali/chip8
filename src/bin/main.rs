use chip8::Chip8;
use std::fs;

fn main() {
    let game_data = match fs::read("./games/MAZE") {
        Ok(data) => data,
        Err(error) => {
            println!("{}", error);
            vec![]
        }
    };

    let mut chip = Chip8::new_with_memory(game_data);
    chip.main_loop();
}

fn print_file(data: Vec<u8>) {
    let mut i = 0;
    while i < data.len() {
        print!("{:02x}", data[i]);
        println!("{:02x}", data[i + 1]);
        i += 2;
    }
}
