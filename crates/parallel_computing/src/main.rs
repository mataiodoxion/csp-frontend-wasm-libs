use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Example {
    name: &'static str,
    code: &'static str,
}

fn main() -> std::io::Result<()> {
    let examples = vec![Example {
        name: "threads_ex",
        code: include_str!("threads_ex.rs"),
    }];

    fs::write(
        "../../pkg/examples.json",
        serde_json::to_string_pretty(&examples).unwrap(),
    )?;
    Ok(())
}
