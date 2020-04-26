use rand::prelude::*;

pub mod utils;

const LAST_4: u16 = 0b0000_0000_0000_1111;
const LAST_8: u16 = 0b0000_0000_1111_1111;
const LAST_12: u16 = 0b0000_1111_1111_1111;

const SCREEN_X: usize = 64;
const SCREEN_Y: usize = 32;

#[cfg(test)]
fn randon_u8() -> u8 {
    0xff
}

#[cfg(not(test))]
fn randon_u8() -> u8 {
    random()
}

pub struct Chip8 {
    memory: [u8; 4096],
    memory_position: usize,
    stack: [usize; 16],
    stack_position: usize,
    i: usize,
    registers: [u8; 16],
    pub screen: [[u8; SCREEN_X]; SCREEN_Y],
    key_pressed: u8,
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
            screen: [[0; SCREEN_X]; SCREEN_Y],
            key_pressed: 0,
        }
    }

    pub fn main_loop(&mut self) {
        let mut should_iterate = true;
        while should_iterate {
            should_iterate = self.single_iteration();
        }
    }

    pub fn single_iteration(&mut self) -> bool {
        let instruction = self.get_next_instruction();
        let address = (instruction & LAST_12) as usize;
        let mask = (instruction & LAST_8) as u8;
        let register_x = ((instruction >> 8) & LAST_4) as usize;
        let register_y = ((instruction >> 4) & LAST_4) as usize;
        let sub_op = (instruction & LAST_4) as usize;

        match instruction {
            0x0000 => return false,
            0x00E0 => self.clear_screen(),
            0x00EE => self.return_from_subroutine(),
            0x1000..=0x1FFF => {
                self.memory_position = address;
                return true;
            }
            0x2000..=0x2FFF => {
                self.execute_subroutine(address);
                return true;
            }
            0x3000..=0x3FFF => self.skip_next_position(self.registers[register_x] == mask),
            0x4000..=0x4FFF => self.skip_next_position(self.registers[register_x] != mask),
            0x5000..=0x5FFF => {
                self.skip_next_position(self.registers[register_x] == self.registers[register_y])
            }
            0x6000..=0x6FFF => self.registers[register_x] = mask,
            0x7000..=0x7FFF => self.registers[register_x] += mask,
            0x9000..=0x9FFF => {
                self.skip_next_position(self.registers[register_x] != self.registers[register_y])
            }
            0xA000..=0xAFFF => self.i = address,
            0xB000..=0xBFFF => {
                self.memory_position = address + (self.registers[0x0] as usize);
                return true;
            }
            0xC000..=0xCFFF => self.set_random_value(register_x, mask),
            0xD000..=0xDFFF => self.draw(
                self.registers[register_x] as usize,
                self.registers[register_y] as usize,
                sub_op,
            ),
            0xE09E..=0xEF9E => {
                self.skip_next_position(self.registers[register_x] == self.key_pressed)
            }
            0xE0A1..=0xEFA1 => {
                self.skip_next_position(self.registers[register_x] != self.key_pressed)
            }
            _ => panic!("UNKNOWN OPCODE: {:04x}", instruction),
        }

        self.memory_position += 2;
        return true;
    }

    fn execute_subroutine(&mut self, address: usize) {
        self.stack[self.stack_position] = self.memory_position;
        self.stack_position += 1;
        self.memory_position = address;
    }

    fn return_from_subroutine(&mut self) {
        if self.stack_position == 0 {
            panic!("stack underflow!");
        }

        self.stack_position -= 1;
        self.memory_position = self.stack[self.stack_position];
    }

    fn skip_next_position(&mut self, should_skip: bool) {
        if should_skip {
            self.memory_position += 2;
        }
    }

    fn set_random_value(&mut self, register_id: usize, mask: u8) {
        let x: u8 = randon_u8();
        self.registers[register_id] = x & mask;
    }

    fn get_next_instruction(&self) -> u16 {
        let instruction: u16 = self.memory[self.memory_position] as u16;
        let instruction_2: u16 = self.memory[self.memory_position + 1] as u16;
        instruction << 8 | instruction_2
    }

    fn clear_screen(&mut self) {
        self.screen = [[0; SCREEN_X]; SCREEN_Y];
    }

    fn draw(&mut self, position_x: usize, position_y: usize, bytes: usize) {
        self.registers[0xF] = 0;

        for i in 0..bytes {
            let data = self.memory[self.i + i];

            self.xor_screen(position_x + 7, position_y + i, (data as u8) & 0b0000_0001);
            self.xor_screen(
                position_x + 6,
                position_y + i,
                ((data as u8) & 0b0000_0010) >> 1,
            );
            self.xor_screen(
                position_x + 5,
                position_y + i,
                ((data as u8) & 0b0000_0100) >> 2,
            );
            self.xor_screen(
                position_x + 4,
                position_y + i,
                ((data as u8) & 0b0000_1000) >> 3,
            );

            self.xor_screen(
                position_x + 3,
                position_y + i,
                ((data as u8) & 0b0001_0000) >> 4,
            );
            self.xor_screen(
                position_x + 2,
                position_y + i,
                ((data as u8) & 0b0010_0000) >> 5,
            );
            self.xor_screen(
                position_x + 1,
                position_y + i,
                ((data as u8) & 0b0100_0000) >> 6,
            );
            self.xor_screen(
                position_x + 0,
                position_y + i,
                ((data as u8) & 0b1000_0000) >> 7,
            );
        }
    }

    fn xor_screen(&mut self, x: usize, y: usize, value: u8) {
        if x >= SCREEN_X || y >= SCREEN_Y {
            return;
        }

        self.screen[y][x] = if self.screen[y][x] == value {
            if self.screen[y][x] == 1 {
                self.registers[0xF] = 0x01;
            }
            0
        } else {
            1
        };
    }

    pub fn get_state(&self) -> String {
        return format!(
            "memory_position: {} ({:02x})\n\
                        i: {} ({:03x})\n\n\
                        v[0..3]:    {:02x} {:02x} {:02x} {:02x}\n\
                        v[4..7]:    {:02x} {:02x} {:02x} {:02x}\n\
                        v[8..11]:   {:02x} {:02x} {:02x} {:02x}\n\
                        v[12..15]:  {:02x} {:02x} {:02x} {:02x}",
            self.memory_position,
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
        )
        .to_string();
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
        let program = vec![0xC2, 0x10];
        let mut chip = Chip8::new_with_memory(program);

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.registers[0x02], 0x10);
    }

    #[test]
    fn should_skip_if_equals_on_3xxx() {
        let program = vec![0x32, 0x01];
        let mut chip = Chip8::new_with_memory(program);
        chip.registers[0x02] = 0x01;

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.memory_position, 0x200 + 4);
    }

    #[test]
    fn should_not_skip_if_different_on_3xxx() {
        let program = vec![0x32, 0x01];
        let mut chip = Chip8::new_with_memory(program);
        chip.registers[0x02] = 0x02;

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.memory_position, 0x200 + 2);
    }

    #[test]
    fn should_not_skip_if_equals_on_4xxx() {
        let program = vec![0x42, 0x01];
        let mut chip = Chip8::new_with_memory(program);
        chip.registers[0x02] = 0x01;

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.memory_position, 0x200 + 2);
    }

    #[test]
    fn should_skip_if_different_on_4xxx() {
        let program = vec![0x42, 0x01];
        let mut chip = Chip8::new_with_memory(program);
        chip.registers[0x02] = 0x02;

        let should_iterate = chip.single_iteration();

        assert_eq!(should_iterate, true);
        assert_eq!(chip.memory_position, 0x200 + 4);
    }
}
