use clap::{Arg, ArgAction};
use walkdir::DirEntry;

pub fn size_argument() -> Arg {
    Arg::new("size")
        .short('s')
        .long("size")
        .action(ArgAction::SetTrue)
        .required(false)
        .help("Display the size of each file and folder in bytes.")
}

pub fn match_entry_size(entry: &DirEntry, size: bool) -> Option<String> {
    match (size, entry.metadata()) {
        (false, _) => None,
        (_, Ok(m)) => {
            let mut bytes = m.len();
            let mut end = 'B';
            if bytes > 1000 {
                bytes /= 1000;
                end = 'K'
            }
            if bytes > 1000 {
                bytes /= 1000;
                end = 'M'
            }
            if bytes > 1000 {
                bytes /= 1000;
                end = 'G'
            }
            if bytes > 1000 {
                bytes /= 1000;
                end = 'P'
            }
            let mut s = bytes.to_string();
            s.push(end);
            Some(s)
        }
        (_, Err(_)) => None,
    }
}
