use argh::*;
use std::path::PathBuf;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "print-config",
    description = "Generate manifest YAML from a list of directories"
)]
pub struct PrintConfigArgs {
    #[argh(
        option,
        long = "container",
        short = 'D',
        description = "containers of build directories"
    )]
    pub containers: Vec<PathBuf>,
    #[argh(
        option,
        long = "repository",
        short = 'T',
        description = "paths to repository files"
    )]
    pub repositories: Vec<PathBuf>,
}
