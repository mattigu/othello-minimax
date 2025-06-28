const RESET: &str = "\x1b[0m";
const LIGHT_CYAN: &str = "\x1b[96m";
const LIGHT_MAGENTA: &str = "\x1b[95m";

pub fn color(text: &str, ansi: &str) -> String {
    format!("{ansi}{text}\x1b[0m")
}

pub fn ansi_for(symbol: char) -> &'static str {
    match symbol {
        'x' => LIGHT_CYAN,
        'o' => LIGHT_MAGENTA,
        _ => RESET,
    }
}
