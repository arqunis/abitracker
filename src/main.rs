//! Utility for printing the statistics of packages that were upgraded today.
//!
//! The statistics are about how many packages were upgraded due to changes in
//! the packages themselves, or had to be rebuilt due to an ABI break in another
//! package.

/// Represents the version of a package.
#[derive(Debug)]
pub struct Version {
    /// The version itself, whose format we do not care about because it can be
    /// arbitrary.
    pub text: String,
    /// Release number specified after the version by a dash.
    pub release: u32,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.text.eq(&other.text)
    }
}

impl Eq for Version {}

/// Given a version, parse it into a [`Version`].
pub fn parse_version(text: &str) -> Version {
    let (text, release) = text.split_once("-").expect("invalid version");

    Version {
        text: text.to_string(),
        release: release.parse().expect("invalid release number"),
    }
}

/// Represents a package that has been upgraded today.
#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub before: Version,
    pub after: Version,
}

/// Given a line from pacman.log about an upgraded package, parse it into a [`Package`].
pub fn parse_package(line: &str) -> Package {
    use regex::Regex;
    use std::sync::OnceLock;

    static UPGRADE_TEXT_REGEX: OnceLock<Regex> = OnceLock::new();

    let regex = UPGRADE_TEXT_REGEX
        .get_or_init(|| Regex::new(r"upgraded\s*([^\s]+)\s*\(([^\s]+)\s*->\s*([^\s]+)\)").unwrap());

    let caps = regex.captures(line)
        .expect("invalid line, does not contain `upgraded`, the name of the package, its prior version, its new version, or everything");

    Package {
        name: caps[1].to_string(),
        before: parse_version(&caps[2]),
        after: parse_version(&caps[3]),
    }
}

#[derive(Debug)]
pub struct Statistics {
    /// Packages whose versions changed.
    pub changed: u64,
    /// Packages whose release numbers changed.
    pub rebuilt: u64,
}

impl Statistics {
    pub fn new(packages: &[Package]) -> Self {
        Self {
            changed: packages.iter().filter(|p| p.before != p.after).count() as u64,
            rebuilt: packages.iter().filter(|p| p.before == p.after).count() as u64,
        }
    }
}

/// Prints the statistics to Standard Output.
pub fn print_statistics(stats: &Statistics) {
    println!("[abitracker]: Packages upgraded today had {} legitimate upgrades, versus {} that had to be rebuilt due to other packages",
        stats.changed,
        stats.rebuilt,
    );
}

/// Read the entire `/var/log/pacman.log` file to memory.
pub fn read_pacman_log() -> String {
    std::fs::read_to_string("/var/log/pacman.log")
        .expect("failed to read `/var/log/pacman.log`, check permissions?")
}

/// Splits string into lines excluding those printed by the program, contain the
/// word "upgraded", and their timestamps are the current date.
fn filter_lines(text: &str) -> impl Iterator<Item = &str> + '_ {
    use chrono::{DateTime, Local};

    let today = Local::now();
    text.lines()
        .filter(|s| !s.contains("abitracker"))
        .filter(|s| s.contains("upgraded"))
        .filter(move |s| {
            let d = s[1..s.bytes().position(|c| c == b']').unwrap()]
                .parse::<DateTime<Local>>()
                .unwrap();
            today.date_naive() == d.date_naive()
        })
}

fn main() {
    let input = read_pacman_log();

    let mut packages = Vec::new();

    for line in filter_lines(&input) {
        packages.push(parse_package(line));
    }

    let stats = Statistics::new(&packages);

    print_statistics(&stats);
}
