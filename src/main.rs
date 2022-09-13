use chip_8_emulator::{Chip8};

fn main() {
    let mut x = Chip8::new();
    x.load_rom_from_radix_at_512(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testroms\\store_mem_test.chip8"));
    
    // x.load_font();
    // // println!("")
    // x.start_processor_loop();

    // let x : u16 = 0xea;
    // let hundreds = x/100;
    // let tens = (x - (hundreds * 100)) /10;
    // let ones = (x - (hundreds * 100) - (tens * 10)) / 1;

    x.v_regs[0] = 0x2c;
    x.bcd_instruction(0);

    // let ones = (x/1) - (hundreds * 100) - (tens * 10);
    println!("{}",x.memory[0]);
    println!("{}",x.memory[1]);
    println!("{}",x.memory[2]);
}