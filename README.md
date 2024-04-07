# Glove80 keymap cli

Read .keymap file for the glove80 and visualize the keyboard.

```text
$ bin/keys default symbols
╭───┬───┬───┬───┬───╮                                         ╭───┬───┬───┬───┬───╮
│F1 │F2 │F3 │F4 │F5 │                 DEFAULT                 │F6 │F7 │F8 │F9 │F10│
├───┼───┼───┼───┼───┼───╮                                 ╭───┼───┼───┼───┼───┼───┤
│F11│ 1 │ 2 │ 3 │ 4 │ 5 │                                 │ 6 │ 7 │ 8 │ 9 │ 0 │F12│
├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤
│   │ Q │ W │ E │ R │ T │                                 │ Y │ U │ I │ O │ P │   │
├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤
│   │ A │ S │ D │ F │ G │                                 │ H │ J │ K │ L │ - │   │
├───┼───┼───┼───┼───┼───┤╭────┬────┬────╮ ╭────┬────┬────╮├───┼───┼───┼───┼───┼───┤
│   │ Z │ X │ C │ V │ B ││ Esc│Ctrl│LAlt│ │NUMB│Ctrl│Ret ││ N │ M │ , │ . │ / │   │
├───┼───┼───┼───┼───┼───╯├────┼────┼────┤ ├────┼────┼────┤╰───┼───┼───┼───┼───┼───┤
│Mag│Hom│End│Lft│Rgt│    │SYMB│Shft│GERM│ │Ins │Bksp│Spce│    │Dwn│Up │PDn│PUp│Sys│
╰───┴───┴───┴───┴───╯    ╰────┴────┴────╯ ╰────┴────┴────╯    ╰───┴───┴───┴───┴───╯
╭───┬───┬───┬───┬───╮                                         ╭───┬───┬───┬───┬───╮
│   │   │   │   │   │                 SYMBOLS                 │   │   │   │   │   │
├───┼───┼───┼───┼───┼───╮                                 ╭───┼───┼───┼───┼───┼───┤
│   │ ! │   │   │   │ % │                                 │ ^ │   │   │   │ ø │   │
├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤
│   │ ? │ = │ { │ ' │ } │                                 │ # │ & │   │   │ + │   │
├───┼───┼───┼───┼───┼───┤                                 ├───┼───┼───┼───┼───┼───┤
│   │ @ │ * │ ( │ " │ ) │                                 │ < │ - │Pip│ > │ - │   │
├───┼───┼───┼───┼───┼───┤╭────┬────┬────╮ ╭────┬────┬────╮├───┼───┼───┼───┼───┼───┤
│   │ € │ $ │ [ │ ` │ ] ││    │    │    │ │    │    │    ││ \ │ _ │ ; │ : │ / │   │
├───┼───┼───┼───┼───┼───╯├────┼────┼────┤ ├────┼────┼────┤╰───┼───┼───┼───┼───┼───┤
│Mag│   │   │   │   │    │    │    │    │ │SIns│Del │STab│    │   │   │   │   │   │
╰───┴───┴───┴───┴───╯    ╰────┴────┴────╯ ╰────┴────┴────╯    ╰───┴───┴───┴───┴───╯
```

## Options

```bash
$ bin/keys --help
keys 
Keymap viewer for the glove80

USAGE:
    keys [OPTIONS] [FILTER]...

ARGS:
    <FILTER>...    One or multiple names of the layout to show. If no filter is specified, the
                   default layout will be shown. Can be substrings of the actual name (e.g. sym
                   for SYMBOLS)

OPTIONS:
    -f, --file <FILE>    Specify the .keymap file to read the layout [default:
                         ../config/config/glove80.keymap]
    -h, --help           Print help information
```

## Compile static bin

```bash
RUSTFLAGS="-Ctarget-feature=+crt-static" cargo build --target=x86_64-unknown-linux-gnu
mv ./target/x86_64-unknown-linux-gnu/debug/keys bin/keys
```

