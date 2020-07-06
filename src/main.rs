use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut lines: Vec<String> = vec![String::new(); 3];

    std::io::stdout().write_all(b"String to write? ")?;
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.len() - 1); // Strip the trailing newline  

    lines[0] = input.chars().fold(String::new(), |acc, letter| acc + &letter_top(&letter));
    lines[1] = input.chars().fold(String::new(), |acc, letter| acc + &letter_middle(&letter));
    lines[2] = input.chars().fold(String::new(), |acc, letter| acc + &letter_bottom(&letter));
    
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

fn letter_top(this_letter: &char) -> String {
    let letter_as_string: &str = &this_letter.to_string();
    return match letter_as_string {
        "a" | "A" => "┌─┐".to_string(),
        "b" | "B" => "┬─┐".to_string(),
        "c" | "C" => "┌─┐".to_string(),
        "d" | "D" => "┬─┐".to_string(),
        "e" | "E" => "┬─┐".to_string(),
        "f" | "F" => "┬─┐".to_string(),
        "g" | "G" => "┌─┐".to_string(),
        "h" | "H" => "┬ ┬".to_string(),
        "i" | "I" => " ┬ ".to_string(),
        "j" | "J" => "  ┬".to_string(),
        "k" | "K" => "╷ ╭".to_string(),
        "l" | "L" => "┬  ".to_string(),
        "m" | "M" => "┌┬┐".to_string(),
        "n" | "N" => "╷ ┬".to_string(),
        "o" | "O" => "┌─┐".to_string(),
        "p" | "P" => "┬─┐".to_string(),
        // q is skipped due to laziness
        "r" | "R" => "┬─┐".to_string(),
        "s" | "S" => "┌─┐".to_string(),
        "t" | "T" => "┌┬┐".to_string(),
        "u" | "U" => "╷ ╷".to_string(),
        "v" | "V" => "╷ ╷".to_string(),
        "w" | "W" => "┬╷┬".to_string(),
        "x" | "X" => "╷ ╷".to_string(),
        "y" | "Y" => "╷ ╷".to_string(),
        "z" | "Z" => "──┐".to_string(),
        default => default.to_string()
    }
}

fn letter_middle(this_letter: &char) -> String {
    let letter_as_string: &str = &this_letter.to_string();
    return match letter_as_string {
        "a" | "A" => "├─┤".to_string(),
        "b" | "B" => "├─┤".to_string(),
        "c" | "C" => "│  ".to_string(),
        "d" | "D" => "│ │".to_string(),
        "e" | "E" => "├─ ".to_string(),
        "f" | "F" => "├─ ".to_string(),
        "g" | "G" => "│ ┬".to_string(),
        "h" | "H" => "├─┤".to_string(),
        "i" | "I" => " │ ".to_string(),
        "j" | "J" => "  │".to_string(),
        "k" | "K" => "├─ ".to_string(),
        "l" | "L" => "│  ".to_string(),
        "m" | "M" => "│││".to_string(),
        "n" | "N" => "│╲│".to_string(),
        "o" | "O" => "│ │".to_string(),
        "p" | "P" => "├─┘".to_string(),
        // q is skipped due to laziness
        "r" | "R" => "├─┤".to_string(),
        "s" | "S" => "└─┐".to_string(),
        "t" | "T" => " │ ".to_string(),
        "u" | "U" => "│ │".to_string(),
        "v" | "V" => "│ │".to_string(),
        "w" | "W" => "│││".to_string(),
        "x" | "X" => " │ ".to_string(),
        "y" | "Y" => "└┬┘".to_string(),
        "z" | "Z" => "┌─┘".to_string(),
        default => default.to_string()
    }
}

fn letter_bottom(this_letter: &char) -> String {
    let letter_as_string: &str = &this_letter.to_string();
    return match letter_as_string {
        "a" | "A" => "╵ ╵".to_string(),
        "b" | "B" => "┴─┘".to_string(),
        "c" | "C" => "└─┘".to_string(),
        "d" | "D" => "┴─┘".to_string(),
        "e" | "E" => "┴─┘".to_string(),
        "f" | "F" => "┴  ".to_string(),
        "g" | "G" => "└─┘".to_string(),
        "h" | "H" => "┴ ┴".to_string(),
        "i" | "I" => " ┴ ".to_string(),
        "j" | "J" => "└─┘".to_string(),
        "k" | "K" => "╵ ╰".to_string(),
        "l" | "L" => "└─┘".to_string(),
        "m" | "M" => "┴╵┴".to_string(),
        "n" | "N" => "┴ ╵".to_string(),
        "o" | "O" => "└─┘".to_string(),
        "p" | "P" => "┴  ".to_string(),
        // q is skipped due to laziness
        "r" | "R" => "┴ ┴".to_string(),
        "s" | "S" => "└─┘".to_string(),
        "t" | "T" => " ┴ ".to_string(),
        "u" | "U" => "└─┘".to_string(),
        "v" | "V" => " ╵ ".to_string(),
        "w" | "W" => "└┴┘".to_string(),
        "x" | "X" => "╵ ╵".to_string(),
        "y" | "Y" => " ╵ ".to_string(),
        "z" | "Z" => "└──".to_string(),
        default => default.to_string()
    }
}