//! # Terminal paint
//! 
//! `terminal-paint` is a library that simplifies printing colorful text to your console.
//! It is based on ANSI Escape Code. Should work on Linux, macOS and Windows (from Windows 10 1511+) 

// pub const RESET: u8 = 0;
pub const BOLD: ColorInstruction = ColorInstruction::One(1);
pub const INVERTED: ColorInstruction = ColorInstruction::One(7);
pub const BLACK: ColorInstruction = ColorInstruction::One(30);
pub const RED: ColorInstruction = ColorInstruction::One(31);
pub const GREEN: ColorInstruction = ColorInstruction::One(32);
pub const YELLOW: ColorInstruction = ColorInstruction::One(33);
pub const BLUE: ColorInstruction = ColorInstruction::One(34);
pub const PURPLE: ColorInstruction = ColorInstruction::One(35);
pub const CYAN: ColorInstruction = ColorInstruction::One(36);
pub const WHITE: ColorInstruction = ColorInstruction::One(37);
pub const GRAY: ColorInstruction = ColorInstruction::One(90);
pub const ON_BLACK: ColorInstruction = ColorInstruction::One(40);
pub const ON_RED: ColorInstruction = ColorInstruction::One(41);
pub const ON_GREEN: ColorInstruction = ColorInstruction::One(42);
pub const ON_YELLOW: ColorInstruction = ColorInstruction::One(43);
pub const ON_BLUE: ColorInstruction = ColorInstruction::One(44);
pub const ON_PURPLE: ColorInstruction = ColorInstruction::One(45);
pub const ON_CYAN: ColorInstruction = ColorInstruction::One(46);
pub const ON_WHITE: ColorInstruction = ColorInstruction::One(47);
pub const BLACK_ON_RED: ColorInstruction = ColorInstruction::Two(30, 41);
pub const BLACK_ON_GREEN: ColorInstruction = ColorInstruction::Two(30, 42);
pub const BLACK_ON_YELLOW: ColorInstruction = ColorInstruction::Two(30, 43);
pub const BLACK_ON_BLUE: ColorInstruction = ColorInstruction::Two(30, 44);
pub const BLACK_ON_PURPLE: ColorInstruction = ColorInstruction::Two(30, 45);
pub const BLACK_ON_CYAN: ColorInstruction = ColorInstruction::Two(30, 46);
pub const BLACK_ON_WHITE: ColorInstruction = ColorInstruction::Two(30, 47);

/// `ColorInstruction` is a helper `enum`,
/// that helps to represent numbers in ANSI Escape Code.
/// `ColorInstruction::One` represents either background or font color;
/// `ColorInstruction::Two` represents combination of both; 
pub enum ColorInstruction {
    One(u8),
    Two(u8, u8)
}

/// Returns `String` with the text of specified color
/// 
/// # Examples
/// 
/// ```
/// use terminal_paint as tp;
/// 
/// let my_str = tp::paint("hello world!", tp::YELLOW);
/// let my_str2 = tp::paint("world hello", tp::ON_RED);
/// println!("{}, {}", my_str, my_str2);
/// ```
pub fn paint(text: impl Into<String>, color: ColorInstruction) -> String {
    match color {
        ColorInstruction::One(font_color) => format!("\x1b[{}m{}\x1b[0m", font_color, text.into()),
        ColorInstruction::Two(font_color, background_color) => format!("\x1b[{};{}m{}\x1b[0m", font_color, background_color, text.into()),
    }
}

/// calls `print!` but with specified color
pub fn color_print(text: impl Into<String>, color: ColorInstruction) {
    print!("{}", paint(text, color))
}

/// calls `println!` but with specified color
pub fn color_println(text: impl Into<String>, color: ColorInstruction) {
    println!("{}", paint(text, color))
}