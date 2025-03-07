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

pub fn log_green(msg: &str) {
    println!("{}", msg.color(GREEN));
}

pub fn log_blue(msg: &str) {
    println!("{}", msg.color(BLUE));
}

pub fn log_red(msg: &str) {
    println!("{}", msg.color(Color::Red));
}
