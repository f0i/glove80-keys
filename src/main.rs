use std::fs::read_to_string;
use clap::Parser;

/// Keymap viewer for the glove80
#[derive(Parser, Debug)]
struct Args {
    /// Specify the .keymap file to read the layout
    #[clap(short, long, default_value = "../config/config/glove80.keymap")]
    file: String,

    /// One or multiple names of the layout to show.
    /// If no filter is specified, the default layout will be shown.
    /// Can be substrings of the actual name (e.g. sym for SYMBOLS)
    #[clap(takes_value = true, multiple = true)]
    filter: Vec<String>,
}


fn main() {
    
    let mut args = Args::parse();
    if args.filter.is_empty() {
        args.filter.push("default".to_string());
    };

    let file = read_to_string(args.file) 
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
            let m = args.filter.iter().find(|f|current.label.to_lowercase().contains(&**f));
            if current.keys.len() >= 6 && m.is_some() {
                layouts.push(current.clone());
                print_layout(current.clone());
            }

            current.keys = vec![];
        }
    }
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

