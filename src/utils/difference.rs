use difference::{Changeset, Difference};

pub fn generate_diff_for_string_files(file_one: &str, file_two: &str) {
    let diff: Changeset = Changeset::new(&file_one, &file_two, "\n");
    for change in diff.diffs {
        match change {
            Difference::Same(_) => {}
            Difference::Add(ref text) => {
                let c = &text[4..];
                println!("\u{001b}[32m+{}", c)
            }
            Difference::Rem(ref text) => {
                println!("\u{001b}[31m-{}", text)
            }
        }
    }
}
