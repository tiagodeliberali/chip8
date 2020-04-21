use rand::prelude::*;

fn debug(text: String) {
    println!("{}", text);
}

const LAST_4: u16 = 0b0000_0000_0000_1111;
const LAST_8: u16 = 0b0000_0000_1111_1111;
const LAST_12: u16 = 0b0000_1111_1111_1111;

pub struct Chip8 {
    memory: [u8; 4096],
    memory_position: usize,
    stack: [u16; 16],
    stack_position: usize,
    i: usize,
    registers: [u8; 16],
}

impl Chip8 {
    pub fn new_with_memory(data: Vec<u8>) -> Chip8 {
        let mut memory: [u8; 4096] = [0; 4096];
        let mut position = 0x200;

        for value in data {
            memory[position] = value;
            position += 1;
        }

        Chip8 {
            memory,
            memory_position: 0x200,
            stack: [0; 16],
            stack_position: 0,
            i: 0,
            registers: [0; 16],
        }
    }

    pub fn main_loop(&mut self) {
        let mut should_iterate = true;
        while should_iterate {
            should_iterate = self.single_iteration();
        }
    }

    fn single_iteration(&mut self) -> bool {
        let instruction = self.get_next_instruction();
        let address = (instruction & LAST_12) as usize;
        let mask = (instruction & LAST_8) as u8;
        let register_x = ((instruction >> 8) & LAST_4) as usize;
        let register_y = ((instruction >> 4) & LAST_4) as usize;
        let sub_op = (instruction & LAST_4) as usize;

        debug(format!("OPCODE: {:04x}", instruction));

        match instruction {
            0x0000 => return false,
            0xA000..=0xAFFF => self.i = address,
            0xC000..=0xCFFF => self.set_random_value(register_x, mask),
            _ => println!("UNKNOWN OPCODE: {:04x}", instruction),
        }

        self.print_state();
        self.memory_position += 2;

        return true;
    }

    fn set_random_value(&mut self, register_id: usize, mask: u8) {
        let x: u8 = random();
        debug(format!("RANDOM NUMBER: {:08b}", x));
        debug(format!("MASK: {:08b}", mask));
        self.registers[register_id] = x & mask;
    }

    fn get_next_instruction(&self) -> u16 {
        let instruction: u16 = self.memory[self.memory_position] as u16;
        let instruction_2: u16 = self.memory[self.memory_position + 1] as u16;
        instruction << 8 | instruction_2
    }

    fn print_state(&self) {
        debug(
            format!("memory_position: {:02x}\ni: {} ({:03x})\nv[0..4]:\t{}\t{}\t{}\t{}\t{}\nv[5..9]:\t{}\t{}\t{}\t{}\t{}\nv[10..15]:\t{}\t{}\t{}\t{}\t{}\t{}", 
                self.memory_position,
                self.i,
                self.i,
                self.registers[0],
                self.registers[1],
                self.registers[2],
                self.registers[3],
                self.registers[4],
                self.registers[5],
                self.registers[6],
                self.registers[7],
                self.registers[8],
                self.registers[9],
                self.registers[10],
                self.registers[11],
                self.registers[12],
                self.registers[14],
                self.registers[14],
                self.registers[15],
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_stop_on_0000() {
        let program = vec![0x00, 0x00];
        let mut chip = Chip8::new_with_memory(program);

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, false);
    }

    #[test]
    fn should_store_address_on_i() {
        let program = vec![0xA2, 0x1E];
        let mut chip = Chip8::new_with_memory(program);

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.i, 0x21E);
    }

    #[test]
    fn should_set_random_value_with_mask_on_registers() {
        // This is a fake test, to avoid deal with random
        // Must implement a Utils/Random trait and import it inside Chip8
        let program = vec![0xC2, 0x00];
        let mut chip = Chip8::new_with_memory(program);

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.registers[0x02], 0x00);
    }
}
