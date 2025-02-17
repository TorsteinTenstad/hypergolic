use colored::{Color, Colorize};

const GREEN: Color = Color::TrueColor {
    r: 123,
    g: 146,
    b: 70,
};

const BLUE: Color = Color::TrueColor {
    r: 108,
    g: 153,
    b: 187,
};

pub fn log_green(msg: String) {
    println!("{}", msg.color(GREEN));
}

pub fn log_blue(msg: String) {
    println!("{}", msg.color(BLUE));
}

pub fn log_red(msg: String) {
    println!("{}", msg.color(Color::Red));
}
