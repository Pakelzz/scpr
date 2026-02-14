use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    /// Name of city 
    pub name: Option<String>,

    /// To make selected city default
    /// Example: scpr surabaya -d
    #[arg(short, long)]
    pub default: bool,

    /// To specify date you want, format date is YYYY-MM-DD
    /// Example: scpr surabaya --date 2026-06-12
    #[arg(long)]
    pub date: Option<String>,

    #[arg(short)]
    pub next: bool
}
