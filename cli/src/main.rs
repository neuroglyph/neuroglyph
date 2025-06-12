// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

use clap::{Parser, Subcommand};
use gitmind::App;

#[derive(Parser)]
#[command(name = "gitmind")]
#[command(about = "Semantic layer for Git-based thought networks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize gitmind in the current repository
    Init,

    /// Create a semantic link between two files
    Link {
        /// Source file
        source: String,
        /// Target file  
        target: String,
        /// Type of link (default: CROSS_REF)
        #[arg(short, long, default_value = "CROSS_REF")]
        r#type: String,
    },

    /// List all semantic links
    List {
        /// Filter by source file
        #[arg(short, long)]
        source: Option<String>,
        /// Filter by target file
        #[arg(short, long)]
        target: Option<String>,
    },

    /// Remove links between files
    Unlink {
        /// Source file (or file to unlink all from)
        source: Option<String>,

        /// Target file to unlink from source
        target: Option<String>,

        /// Remove all links from source
        #[arg(long, conflicts_with = "target")]
        all: bool,

        /// Remove all links to this target
        #[arg(long, value_name = "FILE", conflicts_with_all = ["source", "target", "all"])]
        to: Option<String>,

        /// Only remove links of this type
        #[arg(short = 't', long = "type")]
        link_type: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(std::env::current_dir()?);

    match cli.command {
        Commands::Init => {
            app.init()?;
            println!("Initialized gitmind in {}", app.working_dir.display());
        }
        Commands::Link {
            source,
            target,
            r#type,
        } => match app.link(&source, &target, &r#type) {
            Ok(sha) => {
                println!("Created link: {} -> {} ({})", source, target, sha);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Commands::List { source, target } => match app.list(source.as_deref(), target.as_deref()) {
            Ok(links) => {
                if links.is_empty() {
                    println!("No links found");
                } else {
                    for link in links {
                        println!("{}", link);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Commands::Unlink {
            source,
            target,
            all,
            to,
            link_type,
        } => {
            let result = if let Some(to_file) = to {
                app.unlink_to(&to_file, link_type.as_deref())
            } else if all {
                let src = source.expect("source required when using --all");
                app.unlink_all_from(&src, link_type.as_deref())
            } else {
                let src = source.expect("source required");
                let tgt = target.expect("target required");
                app.unlink(&src, &tgt, link_type.as_deref())
            };

            match result {
                Ok(count) => {
                    if count == 0 {
                        println!("No matching links found");
                    } else {
                        println!(
                            "Removed {} link{}",
                            count,
                            if count == 1 { "" } else { "s" }
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
