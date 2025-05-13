use color_eyre::owo_colors::OwoColorize;
use figlet_rs::FIGfont;

pub fn print_banner() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("rstl");

    assert!(figure.is_some(), "Failed to load font file!");

    println!("{}", figure.unwrap().to_string().bold().blue());
}

pub fn print_label(message: &str, value: &str, depth: i32) {
    println!(
        "{}{}: `{}`",
        "\n".repeat(depth as usize),
        message.bold().green(),
        value
    )
}
