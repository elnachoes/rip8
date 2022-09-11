use chip_8_emulator::{Chip8};

fn main() {
    let mut x = Chip8::new();
    x.load_rom_from_radix(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testroms\\treasure_chest.chip8"));
    x.start_processor_loop();
}