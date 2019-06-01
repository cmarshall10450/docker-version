use clap::ArgMatches;

use regex::Regex;
use semver::Version;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
pub fn bump(matches: &ArgMatches) -> Result<(), String> {
  if matches.args.len() < 1 {
    return Err(String::from("Not enough args passed."));
  }

  let bump_type = match matches.value_of("commit_message").unwrap() {
    s if s.contains("[bump minor]") => "minor",
    s if s.contains("[bump major]") => "major",
    _ => "patch",
  };

  let filename = matches.value_of("filename").unwrap_or("VERSION");
  let mut version = String::new();
  let mut file = File::open(filename).unwrap();
  file.read_to_string(&mut version);

  let (old, new, new_content) = get_and_bump(version, bump_type);

  println!("Bumping version from {} to {}", old, new);

  if matches.is_present("extract") {
    println!("Version: {}", old);
    return Ok(());
  }

  let mut output = OpenOptions::new().write(true).open(filename).unwrap();
  write!(&mut output, "{}", new_content).unwrap();

  Ok(())
}

fn get_and_bump(v: String, bump_type: &str) -> (String, String, String) {
  let re = Regex::new(r"(\d+\.)?(\d+\.)?(\*|\d+)").unwrap();
  let vs = &re.captures(v.as_ref()).unwrap()[0];

  let version = Version::parse(vs).unwrap();
  let mut new_version = version.clone();
  match bump_type {
    "major" => new_version.increment_major(),
    "minor" => new_version.increment_minor(),
    _ => new_version.increment_patch(),
  };
  let new_content = format!("version=\"{}\"", new_version);

  (version.to_string(), new_version.to_string(), new_content)
}