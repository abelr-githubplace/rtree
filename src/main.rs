use std::path::PathBuf;
use tester::{
    command::{get_command, CommandSummary, CommandSummaryBuilder, CURRENT_DIRECTORY},
    sorter::Sortby,
};

fn main() {
    let matches = get_command().get_matches();

    let mut summary_builder = CommandSummaryBuilder::default();
    summary_builder
        .dir(
            matches
                .get_one::<PathBuf>("dir")
                .or(Some(&PathBuf::from(CURRENT_DIRECTORY)))
                .cloned()
                .unwrap(),
        )
        .max(matches.get_one::<usize>("max").cloned())
        .sort(matches.get_one::<Sortby>("sort").cloned())
        .stats(match matches.get_one("stats") {
            Some(&s) => s,
            None => false,
        })
        .tree(match matches.get_one("tree") {
            Some(&t) => t,
            None => false,
        })
        .size(match matches.get_one("size") {
            Some(&s) => s,
            None => false,
        })
        .file(match matches.get_one("file") {
            Some(&f) => f,
            None => false,
        })
        .all(match matches.get_one("all") {
            Some(&a) => a,
            None => false,
        });

    CommandSummary::from(summary_builder).exec();
}
