use colored::Colorize;

/// color_pringln
/// # Example
/// ```
/// color_pringln_green("hello world");
/// ```
pub fn color_pringln_green(text: &str) {
    println!("{}", text.bold().color("green"));
}

pub fn color_pringln(text: &str, mut color: &str) {
    println!("{}", text.bold().color(color));
}
