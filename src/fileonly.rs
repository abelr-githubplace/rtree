use clap::{Arg, ArgAction};

pub fn fileonly_argument() -> Arg {
    Arg::new("file")
        .short('f')
        .long("file-only")
        .action(ArgAction::SetTrue)
        .conflicts_with("tree")
        .required(false)
        .help("Enable file-only listing.")
}
