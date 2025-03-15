use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};

pub fn sorter_argument() -> Arg {
    Arg::new("sort")
        .long("sortby")
        .value_name("sort method")
        .value_parser(value_parser!(Sortby))
        .required(false)
        .help("Sorting method for entries.")
}

#[derive(Clone, Copy)]
pub enum Sortby {
    Name,
    Size,
    DirFirst,
    DirLast,
}

impl ValueEnum for Sortby {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Size, Self::Name, Self::DirFirst, Self::DirLast]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Name => PossibleValue::new("name"),
            Self::Size => PossibleValue::new("size"),
            Self::DirFirst => PossibleValue::new("dirfirst"),
            Self::DirLast => PossibleValue::new("dirlast"),
        })
    }
}

impl std::str::FromStr for Sortby {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for &variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
