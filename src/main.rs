use std::fs::read_to_string;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value = "1")]
    max: u8,

    #[clap(last = true)]
    extra_args: Vec<String>,
}


fn main() {

    let filename = "/workspace/config/config/glove80.keymap";
    // Iterator over each line in the file
    let file = read_to_string(filename) 
        .unwrap();  // panic on possible file-reading errors

    let mut processing = false;
    let mut current = Layout {label : "", keys : vec![]};
    let mut layouts : Vec<Layout> = vec![];
     
    for line in file.lines() {
        if !processing && 
        line.trim_start().starts_with("//") &&
        line.trim().chars().all(|c| c.is_uppercase() || c.is_whitespace() || c == '/')
        {
            processing = true;
            current.label = line.trim_matches(|c| c == ' ' || c == '/');
        }
        else if processing && line.trim().chars().nth(3).unwrap_or(' ') == '|'
        {
            current.keys.push(line.trim());
        }else if processing {
            processing = false;
            if current.keys.len() >= 6 {
                layouts.push(current.clone());
                print_layout(current.clone());
            }

            current.keys = vec![];
        }
    }

    // Find the line starting with "//" and containing only uppercase letters or whitespace
    let label = file.lines().find(|line|
        line.trim_start().starts_with("//") &&
        line.trim().chars().all(|c| c.is_uppercase() || c.is_whitespace() || c == '/')
    ).unwrap().trim().trim_start_matches("// ");

    let input : Vec<&str> = file.lines()
        .map(|line| line.trim())
        .filter(|line| line.chars().nth(3).unwrap_or(' ') == '|')
        .collect();

    //print_layout(label, input.to_vec());
}

#[derive(Clone)]
struct Layout<'a> {
    label: &'a str,
    keys: Vec<&'a str>,
}

fn print_layout(layout : Layout){
    let label = layout.label;
    let input = layout.keys;
    let label_index = 41 - label.len() / 2;

    for i in 0..13 {
        for c in 0..83 {
            let templ = get_template_char(i,c);
            if i == 1 && c >= label_index && c < label_index+label.len() {
                print!("{}", label.trim().chars().nth(c - label_index).unwrap_or(' '));
            } else if i % 2 == 1 && templ == '.' {
                let row = input[i / 2].trim();
                let inp = row.chars().nth(c + 3).unwrap_or('?');
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
