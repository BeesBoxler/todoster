mod args;
mod todo_item;

use args::*;
use regex::RegexBuilder;
use std::{fs, path::PathBuf};
use todo_item::*;

fn main() -> Result<(), ArgumentError> {
    let args = parse_arguments()?;
    let todos = get_todos();

    if args.format == Format::Markdown {
        // todo: tidy this up
        println!(
            "# {}",
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
    }
    todos.iter().for_each(|todo| todo.print(&args.format));

    Ok(())
}

fn parse_arguments() -> Result<Args, ArgumentError> {
    let mut args = std::env::args();
    let mut arguments = Args::default();

    while let Some(arg) = args.next() {
        // TODO: This will be removed once more options are added
        #[allow(clippy::single_match)]
        match &arg[..] {
            "--format" => arguments.format = Format::try_from(args.next().unwrap())?,
            _ => {}
        }
    }

    Ok(arguments)
}

fn get_todos() -> Vec<TodoItem> {
    let re = RegexBuilder::new(r"(?:#+|\/{2,})\s+TODO:\s(?<title>.*)")
        .case_insensitive(true)
        .build()
        .unwrap();
    let mut paths = vec![];
    get_file_paths(PathBuf::from("."), &mut paths);

    let mut todos = vec![];

    for path in paths {
        if let Ok(file) = fs::read_to_string(&path) {
            file.lines().enumerate().for_each(|(line_no, line)| {
                if re.is_match(line) {
                    let captures = re.captures(line).unwrap();
                    if let Some(title) = captures.name("title") {
                        todos.push(TodoItem::new(
                            path.clone(),
                            line_no,
                            title.as_str().to_string(),
                        ));
                    }
                }
            })
        }
    }

    todos
}

fn get_file_paths(path: PathBuf, paths: &mut Vec<PathBuf>) {
    const IGNORE: [&str; 4] = ["target", "node_modules", ".git", "dist"];

    if let Ok(list) = fs::read_dir(path) {
        for item in list {
            let item = item.unwrap();
            if !IGNORE.contains(&item.file_name().to_str().unwrap()) {
                if item.metadata().unwrap().is_dir() {
                    get_file_paths(item.path(), paths);
                } else {
                    paths.push(item.path());
                }
            }
        }
    } else {
        eprintln!("Error! Could not read from current directory");
        std::process::exit(1);
    }
}
