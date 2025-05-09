use colored::Colorize;
use figlet_rs::FIGfont;

pub fn print_banner() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("rstl");

    assert!(figure.is_some(), "Failed to load font file!");

    println!("{}", figure.unwrap().to_string().bold().bright_blue());
}

pub fn print_label(label: &str, message: &str, depth: i32) {
    println!(
        "{}{} `{}`",
        "\t".repeat(depth as usize),
        label.bold().bright_green(),
        message.normal()
    );
}

pub fn print_error(message: &str, depth: i32) {
    println!(
        "{}{} `{}`",
        "\t".repeat(depth as usize),
        "Error".bold().bright_red(),
        message.normal()
    )
}

pub fn print_warning(message: &str, depth: i32) {
    println!(
        "{}{} `{}`",
        "\t".repeat(depth as usize),
        "Warning".bold().bright_yellow(),
        message.normal()
    );
}
