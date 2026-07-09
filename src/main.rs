use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, SelectView};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

struct ProjectInfo<'a> {
    projectname: &'a str,
    git: bool,
    language: &'a str,
    packagemanager: &'a str,
    directory: PathBuf,
}

fn input_step(siv: &mut Cursive) {
    let mut select_view = SelectView::new().popup();

    select_view.add_item("RUST", 1);
    select_view.add_item("GO", 2);
    select_view.add_item("PYTHON", 3);

    siv.add_layer(
        Dialog::new()
            .title("Project Planner")
            .content(
                ListView::new()
                    .child("projectname:", EditView::new().with_name("projectname"))
                    .child("git: ", Checkbox::new().with_name("git"))
                    .child("Language: ", select_view.with_name("option")),
            )
            .button("Ok", |s| {
                let projectname = s
                    .call_on_name("projectname", |t: &mut EditView| t.get_content())
                    .unwrap();
                let git = s
                    .call_on_name("git", |c: &mut Checkbox| c.is_checked())
                    .unwrap();
                let option = s
                    .call_on_name("option", |v: &mut SelectView<i32>| {
                        v.selection().map(|rc| *rc).unwrap_or(1)
                    })
                    .unwrap();
                let packagemanager = pkgmanager(option);
                let language = language(option);
                let directory = env::current_dir().unwrap();
                let options = ProjectInfo {
                    projectname: &projectname,
                    git,
                    language,
                    packagemanager,
                    directory,
                };
                setuproject(s, &options)
            }),
    )
}
fn setuproject(siv: &mut Cursive, options: &ProjectInfo) {
    let path = options.directory.join(options.projectname);
    if path.exists() {
        siv.pop_layer();
        siv.add_layer(
            Dialog::text(format!("Error: Path Already Exists"))
                .title("Error")
                .button("OK", |s| s.quit()),
        );
    }
    if options.language == "Rust" {
        let output = Command::new("cargo")
            .arg("new")
            .arg(options.projectname)
            .output()
            .expect("failed to execute process");
    } else if options.language == "Go" {
        fs::create_dir(options.projectname).expect("failed to create project directory");
        let output = Command::new("go")
            .arg("mod")
            .arg("init")
            .arg(options.projectname)
            .current_dir(options.projectname)
            .output()
            .expect("failed to execute process");
    } else if options.language == "Python" {
        let output = Command::new("uv")
            .arg("init")
            .arg(options.projectname)
            .output()
            .expect("failed to execute process");
    }
    if options.git {
        let output = Command::new("git")
            .arg("init")
            .current_dir(options.projectname)
            .output()
            .expect("failed to execute process");
    };
    siv.pop_layer();
    siv.add_layer(
        Dialog::text(format!(
            "Project Made At: {:?}/{}",
            options.directory, options.projectname
        ))
        .title("Result")
        .button("Quit", |s| s.quit()),
    );
}

fn pkgmanager(option: i32) -> &'static str {
    let a = option;
    match a {
        1 => return "CARGO",
        2 => return "GO MODULES (Default Package Manager)",
        3 => return "UV",
        _ => return "Undefined",
    }
}

fn language(option: i32) -> &'static str {
    match option {
        1 => return "Rust",
        2 => return "Go",
        3 => return "Python",
        _ => return "Undefined",
    }
}

fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv);
    siv.run();
}
