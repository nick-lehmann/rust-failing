use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(display("Could not read file {path}"))]
struct ConfigFileError {
    source: std::io::Error,
    path: String,
}

fn read_config_file(path: &str) -> Result<String, ConfigFileError> {
    std::fs::read_to_string(path).context(ConfigFileSnafu { path })
}

pub fn main() {
    let result = read_config_file("/tmp/does_not_exist.txt");
    println!("{:?}", result);
}
