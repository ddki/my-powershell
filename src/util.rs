use console::style;

pub fn print_line_default() {
    print_line("-", 80, "\n");
}

pub fn print_line_title_default(title: &str) {
    print_line_title(&format!("{}", style(title).bold().yellow().dim()), 80);
}

pub fn print_line_title(title: &str, len: u32) {
    let character_len = (len - title.len() as u32) / 2;
    print_line("-", character_len, "");
    print!("{}", title);
    print_line("-", character_len, "\n")
}

pub fn print_line(character: &str, len: u32, last_esc: &str) {
    for i in 0..len {
        if i == len-1 {
            print!("{}{}", character, last_esc)
        } else {
            print!("{}", character)
        }
    }
}