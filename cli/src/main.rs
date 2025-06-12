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
    },

    /// List all semantic links
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(std::env::current_dir()?);

    match cli.command {
        Commands::Init => {
            app.init()?;
            println!("Initialized gitmind in {}", app.working_dir.display());
        }
        Commands::Link { source, target } => {
            todo!("Implement link command: {} -> {}", source, target);
        }
        Commands::List => {
            todo!("Implement list command");
        }
    }

    Ok(())
}
