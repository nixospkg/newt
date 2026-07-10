use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, SelectView};
use std::env;
use std::fs;
use std::fs::File;
use std::hint::select_unpredictable;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Copy, Clone, PartialEq)]
enum Langs {
    Rust,
    Go,
    Python,
}

#[derive(Copy, Clone)]
enum License {
    Mit,
    Gpl,
    Apache,
    Unlicense,
}

#[derive(Copy, Clone)]
enum PkgManager {
    GoMod,
    Cargo,
    Poetry,
    Pip,
    Uv,
}

#[derive(Clone)]
struct ProjectInfo {
    projectname: String,
    language: Langs,
}

#[derive(Clone)]
struct FullConfig {
    info: ProjectInfo,
    git: bool,
    pkgmanager: PkgManager,
    license: License,
    dir: PathBuf,
}

fn input_step(siv: &mut Cursive) {
    let mut select_view = SelectView::new().popup();

    select_view.add_item("RUST", Langs::Rust);
    select_view.add_item("GO", Langs::Go);
    select_view.add_item("PYTHON", Langs::Python);

    siv.add_layer(
        Dialog::new()
            .title("Newt")
            .content(
                ListView::new()
                    .child("Project Name:", EditView::new().with_name("projectname"))
                    .child("Language: ", select_view.with_name("language_selector")),
            )
            .button("Ok", |s| {
                let projectname = s
                    .call_on_name("projectname", |t: &mut EditView| t.get_content())
                    .unwrap();
                let language = s
                    .call_on_name("language_selector", |v: &mut SelectView<Langs>| {
                        v.selection().map(|rc| *rc).unwrap()
                    })
                    .unwrap();

                let info = ProjectInfo {
                    projectname: projectname.to_string(),
                    language,
                };
                s.pop_layer();
                langandgit(s, info);
            }),
    )
}

fn langandgit(siv: &mut Cursive, info: ProjectInfo) {
    let mut selectpkgmanager = SelectView::new().popup();

    match info.language {
        Langs::Rust => {
            selectpkgmanager.add_item("Cargo", PkgManager::Cargo);
        }
        Langs::Python => {
            selectpkgmanager.add_item("Poetry", PkgManager::Poetry);
            selectpkgmanager.add_item("Pip", PkgManager::Pip);
            selectpkgmanager.add_item("Uv", PkgManager::Uv);
        }
        Langs::Go => {
            selectpkgmanager.add_item("Go Modules", PkgManager::GoMod);
        }
    }

    siv.add_layer(
        Dialog::new()
            .title("Project Planner")
            .content(
                ListView::new()
                    .child("Git: ", Checkbox::new().with_name("git"))
                    .child(
                        "Package Manager: ",
                        selectpkgmanager.with_name("pkg_selector"),
                    ),
            )
            .button("Ok", move |s| {
                let git = s
                    .call_on_name("git", |c: &mut Checkbox| c.is_checked())
                    .unwrap();
                let pkgmanager = s
                    .call_on_name("pkg_selector", |v: &mut SelectView<PkgManager>| {
                        v.selection().map(|rc| *rc).unwrap()
                    })
                    .unwrap();
                s.pop_layer();
                license_step(s, info.clone(), git, pkgmanager);
            }),
    )
}

fn license_step(siv: &mut Cursive, info: ProjectInfo, git: bool, pkgmanager: PkgManager) {
    let mut select_license = SelectView::new().popup();

    select_license.add_item("MIT (Will use YOUR system name)", License::Mit);
    select_license.add_item("GPL 3.0", License::Gpl);
    select_license.add_item("Apache License 2.0", License::Apache);
    select_license.add_item("Unlicensed", License::Unlicense);

    siv.add_layer(
        Dialog::new()
            .title("Project Planner")
            .content(
                ListView::new().child("License: ", select_license.with_name("license_selector")),
            )
            .button("Ok", move |s| {
                let license = s
                    .call_on_name("license_selector", |v: &mut SelectView<License>| {
                        v.selection().map(|rc| *rc).unwrap()
                    })
                    .unwrap();
                let config = FullConfig {
                    info: info.clone(),
                    git,
                    pkgmanager,
                    license,
                    dir: env::current_dir().unwrap(),
                };
                s.pop_layer();
                summary(s, config);
            }),
    )
}

fn setuplicense(license: &str, dir: &PathBuf, projectname: &String) {
    let name = env::var("USER").unwrap_or_else(|_| "unknown".to_string());

    let path = dir.join(&projectname);
    if license == "MIT" {
        let mut licenseinfo = String::from(
            "MIT License\n\nCopyright (c) [year] [fullname]\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\nof this software and associated documentation files (the \"Software\"), to deal\nin the Software without restriction, including without limitation the rights\nto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\ncopies of the Software, and to permit persons to whom the Software is\nfurnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice shall be included in all\ncopies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\nFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\nAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\nLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\nOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\nSOFTWARE.\n",
        );
        licenseinfo = licenseinfo.replace("[year]", "2026");
        licenseinfo = licenseinfo.replace("[fullname]", &name);
        let mut file = File::create("license.txt");
        fs::write(path.join("LICENSE"), licenseinfo).expect("Failed to write LICENSE");
    } else if license == "GPL 3.0" {
        let licenseinfo = include_str!("licenses/gpl-3.0.txt");
        let mut file = File::create(path.join("LICENSE")).expect("Failed to write");
        fs::write(path.join("LICENSE"), licenseinfo).expect("FAILED TO WRITE LICENSE");
    } else if license == "Apache License 2.0" {
        let licenseinfo = include_str!("licenses/apache-2.0.txt");
        let mut file = File::create(path.join("LICENSE")).expect("Failed to write");
        fs::write(path.join("LICENSE"), licenseinfo).expect("FAILED TO WRITE LICENSE");
    } else {
        let licenseinfo = include_str!("licenses/un.txt");
        let mut file = File::create(path.join("LICENSE")).expect("FAILED TO WRITE");
        fs::write(path.join("LICENSE"), licenseinfo).expect("FAILED TO WRITE LICENSE");
    }
}
fn setupkgmanager(manager: &str, git: &bool, dir: &PathBuf, projectname: &String) {
    if manager == "Cargo" {
        let output = Command::new("cargo")
            .arg("init")
            .arg(projectname)
            .current_dir(dir)
            .output()
            .expect("failed to execute process");
    } else if manager == "Uv" {
        let output = Command::new("uv")
            .arg("init")
            .arg(projectname)
            .current_dir(dir)
            .output()
            .expect("failed to execute process");
    } else if manager == "Poetry" {
        let output = Command::new("poetry")
            .arg("new")
            .arg(projectname)
            .current_dir(dir)
            .output()
            .expect("failed to execute process");
    } else if manager == "Pip" {
        let projectpath = dir.join(projectname);
        fs::create_dir(&projectpath).expect("failed to create project directory");
        let output = Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(".venv")
            .current_dir(&projectpath)
            .output()
            .expect("failed to execute process");
        fs::write(projectpath.join("main.py"), "print(\"Hello, world!\")\n")
            .expect("failed to create main.py");
    }

    if *git {
        let output = Command::new("git")
            .arg("init")
            .current_dir(dir.join(projectname))
            .output()
            .expect("failed to execute process");
    }
}
fn setuproject(path: &PathBuf) {
    fs::create_dir(path).unwrap();
}

fn summary(siv: &mut Cursive, config: FullConfig) {
    let path = &config.dir.join(&config.info.projectname);

    if path.exists() {
        siv.pop_layer();
        siv.add_layer(
            Dialog::text("Error: Path Already Exists")
                .title("Error")
                .button("OK", |s| s.quit()),
        );
        return;
    }
    let license = license_name(config.license);
    let project = &config.info.projectname;
    let dir = &config.dir;
    let git = &config.git;
    let pkgmanager = &pkgmanager_name(config.pkgmanager);
    setuproject(path);

    setupkgmanager(pkgmanager, git, dir, project);
    setuplicense(license, dir, project);

    siv.add_layer(
        Dialog::text(format!(
            "Project: {}\nLanguage: {}\nGit: {}\nPackage Manager: {}\nLicense: {}\nDirectory: {:?}",
            config.info.projectname,
            language(config.info.language),
            config.git,
            pkgmanager,
            license,
            dir,
        ))
        .title("Result")
        .button("Quit", |s| s.quit()),
    );
}

fn pkgmanager_name(option: PkgManager) -> &'static str {
    match option {
        PkgManager::Cargo => "Cargo",
        PkgManager::GoMod => "Go Modules",
        PkgManager::Poetry => "Poetry",
        PkgManager::Pip => "Pip",
        PkgManager::Uv => "Uv",
    }
}

fn language(option: Langs) -> &'static str {
    match option {
        Langs::Rust => "Rust",
        Langs::Go => "Go",
        Langs::Python => "Python",
    }
}

fn license_name(option: License) -> &'static str {
    match option {
        License::Mit => "MIT",
        License::Gpl => "GPL 3.0",
        License::Apache => "Apache License 2.0",
        License::Unlicense => "Unlicensed",
    }
}

fn main() {
    let mut siv = cursive::default();
    input_step(&mut siv);
    siv.run();
}
