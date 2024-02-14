use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(default_value_t = { ".".to_string() })]
    path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let a = Args::parse();

    let mut dir = "./".to_string();
    match a.path.as_str() {
        "." => {}
        p => dir.push_str(p),
    }

    println!("{dir}");
    for path in fs::read_dir(dir)? {
        println!("Name: {}", path.unwrap().path().display())
    }

    Ok(())
}
