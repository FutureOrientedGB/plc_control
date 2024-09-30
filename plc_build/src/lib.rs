use regex::Regex;

static VERSION_RS_PATH: &str = "./src/version.rs";
static CARGO_TOML_PATH: &str = "./Cargo.toml";

static REGEX_APP_NAME: &str = r#"(pub static APP_NAME: &str = ")(.+)(";)"#;
static REGEX_APP_VERSION: &str = r#"(pub static APP_VERSION: &str = ")(.+)(";)"#;
static REGEX_PACKAGE_NAME_VERSION: &str =
    r#"(\[package\]\s*\nname\s*=\s*")([^"]+)("\s*\nversion\s*=\s*")([^"]+)"#;

pub fn replace_app_name_version() {
    let package_name = replace_cargo_toml();
    replace_version_rs(package_name);
}

fn replace_version_rs(package_name: String) {
    if let Ok(old_text) = std::fs::read_to_string(&VERSION_RS_PATH) {
        // replace app name
        let pattern_cargo = Regex::new(REGEX_APP_NAME).unwrap();
        let new_text = pattern_cargo.replace_all(&old_text, |caps: &regex::Captures| {
            format!("{}{}{}", &caps[1], package_name, &caps[3])
        });

        // replace app version
        let pattern_git = Regex::new(REGEX_APP_VERSION).unwrap();
        let new_text = pattern_git.replace_all(&new_text, |caps: &regex::Captures| {
            format!(
                "{}{}.{}{}",
                &caps[1],
                get_latest_git_commit_hash(true),
                get_latest_git_commit_date_time(),
                &caps[3]
            )
        });

        if old_text != new_text {
            std::fs::write(&VERSION_RS_PATH, new_text.as_ref()).unwrap();
        }
    } else {
        let mut file = std::fs::File::create(VERSION_RS_PATH).unwrap();
        let text = format!(
            r#"pub static APP_NAME: &str = "{}";{}pub static APP_VERSION: &str = "{}.{}";{}"#,
            package_name,
            "\n",
            get_latest_git_commit_hash(true),
            get_latest_git_commit_date_time(),
            "\n",
        );
        std::io::Write::write_all(&mut file, text.as_ref()).unwrap();
    }
}

fn replace_cargo_toml() -> String {
    // replace package version
    let old_text = std::fs::read_to_string(CARGO_TOML_PATH).unwrap();
    let pattern: Regex = Regex::new(REGEX_PACKAGE_NAME_VERSION).unwrap();
    let new_text = pattern.replace_all(&old_text, |caps: &regex::Captures| {
        format!("{}{}{}{}", &caps[1], &caps[2], &caps[3], get_latest_git_commit_date())
    });
    if old_text != new_text {
        std::fs::write(&CARGO_TOML_PATH, new_text.as_ref()).unwrap();
    }

    // find package name
    return Regex::new(REGEX_PACKAGE_NAME_VERSION)
        .unwrap()
        .captures(&new_text)
        .unwrap()
        .get(2)
        .unwrap()
        .as_str()
        .to_string();
}

fn get_latest_git_commit_hash(short: bool) -> String {
    let output = std::process::Command::new("git")
        .args(&[
            "log",
            "-1",
            if short {
                "--pretty=format:%h"
            } else {
                "--pretty=format:%H"
            },
        ])
        .output()
        .unwrap();

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn get_latest_git_commit_date_time() -> String {
    let output = std::process::Command::new("git")
        .args(&["log", "-1", "--format=%ad", "--date=format:%Y%m%d.%H%M%S"])
        .output()
        .unwrap();

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn get_latest_git_commit_date() -> String {
    let output = std::process::Command::new("git")
        .args(&["log", "-1", "--format=%ad", "--date=format:%Y %m %d"])
        .output()
        .unwrap();

    return String::from_utf8_lossy(&output.stdout)
        .trim()
        .split_whitespace()
        .map(|v| v.trim().parse::<u32>().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(".");
}
