pub struct Font {
    pub font_data : Vec<u8>,

    pub font_location_in_memory : usize,

    pub char_sprite_locations : Vec<u16>,

    pub font_height : u8,
}

impl Font {

    /// this is the new standard font chars and should be stored  
    /// 
    /// the sprites for the characters should be stored at 0x50 - 0x9f
    /// 
    /// add in custom font elsewhere 
    pub fn new_standard() -> Font {
        Font {
            // this is the entire font data set
            font_data : vec![
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 @ 0x50
                0x20, 0x60, 0x20, 0x20, 0x70, // 1 @ 0x55
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 @ 0x5A
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3 @ 0x5F
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4 @ 0x64
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 @ 0x69
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6 @ 0x6E
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7 @ 0x73
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8 @ 0x78
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9 @ 0x7D
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A @ 0x82
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B @ 0x87
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C @ 0x8C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D @ 0x91
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E @ 0x96
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F @ 0x9B
            ],

            // this is the location of where the font memory will be stored
            font_location_in_memory : 0x50,

            // these are the locations of the first bytes in program memory


            char_sprite_locations : vec![
                0x50, // 0 @ 0x50
                0x55, // 1 @ 0x55
                0x5A, // 2 @ 0x5A
                0x5F, // 3 @ 0x5F
                0x64, // 4 @ 0x64
                0x69, // 5 @ 0x69
                0x6E, // 6 @ 0x6E
                0x73, // 7 @ 0x73
                0x78, // 8 @ 0x78
                0x7D, // 9 @ 0x7D
                0x82, // A @ 0x82
                0x87, // B @ 0x87
                0x8C, // C @ 0x8C
                0x91, // D @ 0x91
                0x96, // E @ 0x96
                0x9B, // F @ 0x9B
            ],

            // the font should be 5 pixels tall
            font_height : 5,
        }
    }
}