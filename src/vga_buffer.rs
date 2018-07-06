use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8 ) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    //represents a single character and its color
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    //to be pointed to vga buffer location, will only be written to
    chars:[[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer{
    pub fn write_byte(&mut self, byte: u8){
        match byte{
            b'\n' => self.new_line(),
            byte => { //equal to self catches all other cases(?) (why not default case?)
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character:byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            //vga buffer supports only ascii and code page 437, so we'll have to ensure we're printing valid bytes

            match byte {
                //valid ascii byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                //not part of printable ascii bytes
                _ => self.write_byte(0xfe) //'unknown' block character
            }
        }
    }

    fn new_line(&mut self){
        //TODO
    }

    pub fn write_string_centered(&mut self, s: &str){
        self.column_position = (BUFFER_WIDTH - s.bytes().count()) / 2;
        self.write_string(s)
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_use(core::env)]
pub fn print_boot_msg(){
    use core::fmt::Write;

    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black), //fg and bg color
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}, //points to vga buffer location
    };

    writer.write_string_centered("Successfully booted spren_os");

    write!(writer, " version {}", env!("CARGO_PKG_VERSION")).unwrap();
}