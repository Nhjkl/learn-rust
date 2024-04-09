use colored::Colorize;

pub fn color_pringln_green(text: &str) {
    println!("{}", text.bold().color("green"));
}

pub fn color_pringln(text: &str, mut color: &str) {
    println!("{}", text.bold().color(color));
}
