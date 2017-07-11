use std::io;

use clap;
use serde_yaml;
use serde_json;


error_chain! {
    foreign_links {
        Io(io::Error);
        Clap(clap::Error);
        Yaml(serde_yaml::Error);
        Json(serde_json::Error);
    }
}
