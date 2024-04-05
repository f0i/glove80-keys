fn main() {

    let input = [
        "         //F|F1 |F2 |F3 |F4 |F5 | __ | __ | __ | __ | | __ | __ | __ | __ |F6 |F7 |F8 |F9 |F10|",
        "         //N|F11| 1 | 2 | 3 | 4 | 5  | __ | __ | __ | | __ | __ | __ |  6 | 7 | 8 | 9 | 0 |F12|",
        "         // |   | Q | W | E | R | T  | __ | __ | __ | | __ | __ | __ |  Y | U | I | O | P |   |",
        "         //>|   | A | S | D | F | G  | __ | __ | __ | | __ | __ | __ |  H | J | K | L | - |   |",
        "         // |   | Z | X | C | V | B  | Esc|Ctrl|LAlt| |NUMB|Ctrl|Ret |  N | M | , | . | / |   |",
        "         //L|Mag|Hom|End|Lft|Rgt| __ |SYMB|Shft|GERM| |Ins |Bksp|Spce| __ |Dwn|Up |PDn|PUp|Sys|",
    ];

    let label = "DEFAULT";
    print_layout(label, input.to_vec());
}

fn print_layout(label: &str, input: Vec<&str>){
    let label_index = 41 - label.len() / 2;

    for i in 0..13 {
        for c in 0..83 {
            let templ = get_template_char(i,c);
            if i == 1 && c >= label_index && c < label_index+label.len() {
                print!("{}", label.trim().chars().nth(c - label_index).unwrap_or(' '));
            } else if i % 2 == 1 && templ == '.' {
                let row = input[i / 2].trim();
                let inp = row.chars().nth(c + 3).unwrap();
                print!("{}", inp);
            } else {
                print!("{}", templ);
            }
        }
        println!();
    }
}

fn get_template_char(r:usize, c: usize) -> char{
    let mut template = [
        "╭───┬───┬───┬───┬───╮                                         ╭───┬───┬───┬───┬───╮".chars(),
        "│...│...│...│...│...│                                         │...│...│...│...│...│".chars(),
        "├───┼───┼───┼───┼───┼───╮                                 ╭───┼───┼───┼───┼───┼───┤".chars(),
        "│...│...│...│...│...│...│                                 │...│...│...│...│...│...│".chars(),
        "├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤".chars(),
        "│...│...│...│...│...│...│                                 │...│...│...│...│...│...│".chars(),
        "├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤".chars(),
        "│...│...│...│...│...│...│                                 │...│...│...│...│...│...│".chars(),
        "├───┼───┼───┼───┼───┼───┤╭────┬────┬────╮ ╭────┬────┬────╮├───┼───┼───┼───┼───┼───┤".chars(),
        "│...│...│...│...│...│...││....│....│....│ │....│....│....││...│...│...│...│...│...│".chars(),
        "├───┼───┼───┼───┼───┼───╯├────┼────┼────┤ ├────┼────┼────┤╰───┼───┼───┼───┼───┼───┤".chars(),
        "│...│...│...│...│...│    │....│....│....│ │....│....│....│    │...│...│...│...│...│".chars(),
        "╰───┴───┴───┴───┴───╯    ╰────┴────┴────╯ ╰────┴────┴────╯    ╰───┴───┴───┴───┴───╯".chars(),
    ];
    return template[r].nth(c).unwrap();
}
