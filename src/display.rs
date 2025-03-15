use crate::{command::CommandSummary, visitor::DirVisitor, walking_directories::get_dir_entries};

fn wrapped_display_all(visitor: &DirVisitor) {
    if visitor.get_max_depth() == 0 {
        return;
    }

    let (walk, _) = get_dir_entries(visitor);
    for entry in walk.iter() {
        let path = entry.path().to_owned();

        visitor.string.push_str(&format!(
            "{:>width$}{space}{}\n",
            sum.size.unwrap_or(""),
            path,
            width = if sum.size.is_some() { 5 } else { 0 },
            space = if sum.size.is_some() { " " } else { "" }
        ));

        if path.is_file() {
            *file_count += 1;
        }

        if entry.file_type().is_dir() && m > 1 {
            let new_sum = CommandSummary {
                dir: path,
                max: if sum.max.is_none() || m == 0 {
                    None
                } else {
                    Some(m - 1)
                },
                ..*sum
            };
            display_all(new_sum);
        }
    }
}
