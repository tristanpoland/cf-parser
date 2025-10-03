use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use colored::*;
use serde::Deserialize;
use serde_yaml;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to cf-deployment.yml
    #[arg(short, long, default_value = "cf-deployment.yml")]
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    releases: Option<Vec<Release>>,
    stemcells: Option<Vec<Stemcell>>,
}

#[derive(Debug, Deserialize)]
struct Release {
    name: String,
    version: String,
    url: Option<String>,
    sha1: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Stemcell {
    alias: String,
    os: String,
    version: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Read file
    let mut contents = String::new();
    File::open(&args.file)?.read_to_string(&mut contents)?;
    let manifest: Manifest = serde_yaml::from_str(&contents)?;

    println!("{}", "CF Deployment Releases".bold().underline().cyan());

    if let Some(releases) = manifest.releases {
        println!(
            "{:<25} {:<15} {:<50}",
            "Release".bold(),
            "Version".bold(),
            "URL".bold()
        );

        for r in releases {
            println!(
                "{:<25} {:<15} {:<50}",
                r.name.green(),
                r.version.yellow(),
                r.url.unwrap_or_else(|| "-".to_string()).blue()
            );
        }
    } else {
        println!("{}", "No releases found.".red());
    }

    println!("\n{}", "Stemcells".bold().underline().cyan());
    if let Some(stemcells) = manifest.stemcells {
        println!(
            "{:<15} {:<15} {:<15}",
            "Alias".bold(),
            "OS".bold(),
            "Version".bold()
        );
        for s in stemcells {
            println!(
                "{:<15} {:<15} {:<15}",
                s.alias.green(),
                s.os.yellow(),
                s.version.blue()
            );
        }
    } else {
        println!("{}", "No stemcells found.".red());
    }

    Ok(())
}
