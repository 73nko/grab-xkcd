use clap::Clap;

/// A utility to grab XKCD comics
#[derive(Clap)]
pub struct Args {
    /// Ser a connection timeout
    #[clap(long, short, default_value = "30")]
    pub timeout: u64,

    /// Print output in a format
    #[clap(long, short, arg_enum, default_value = "text")]
    pub output: OutFormat,

    /// The comic to load
    #[clap(long, short, default_value = "0")]
    pub num: usize,

    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,

}

#[derive(Clap, Copy, Clone)]
pub enum OutFormat {
    Json,
    Text,
}