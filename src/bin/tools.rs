use std::{
    fs::{self},
    path::Path,
};

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "tools", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    New {
        #[clap(long)]
        url: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::New { url } => scaffold_new_challenge(&url),
    }
}

fn scaffold_new_challenge(url: &str) -> anyhow::Result<()> {
    let host = extract_host(url).ok_or_else(|| anyhow!("could not parse host from url"))?;

    match host {
        "leetcode.com" => scaffold_leetcode_challenge(url)?,
        _ => bail!("unsupported host: {host}"),
    }

    Ok(())
}

fn scaffold_leetcode_challenge(url: &str) -> anyhow::Result<()> {
    let name = parse_slug(url, "/problems/")
        .ok_or_else(|| anyhow!("Could not parse problem slug from URL."))?;

    let snake_name = name.replace("-", "_");
    let file_path = format!("src/leetcode/{snake_name}.rs");
    if Path::new(&file_path).exists() {
        bail!("{file_path} already exists");
    }

    fs::write(&file_path, template(&snake_name))?;

    println!("Created: {file_path}");
    Ok(())
}

fn extract_host(url: &str) -> Option<&str> {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host_and_port = after_scheme.split('/').next()?;
    let host = host_and_port.split(':').next()?;
    Some(host)
}

fn parse_slug<'a>(url: &'a str, res: &str) -> Option<&'a str> {
    let rest = url.split(res).nth(1)?;
    let slug = rest.split('/').next()?;
    if slug.is_empty() { None } else { Some(slug) }
}

fn template(name: &str) -> String {
    format!(
        r#"pub fn {name}() {{
    todo!()
}}

#[cfg(test)]
mod tests {{
    use super::*;
}}
"#
    )
}
