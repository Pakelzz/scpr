use clap::Parser;
use terminal_size::{Height, terminal_size};

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    /// Name of city
    pub name: Option<String>,

    /// Make city default
    /// Example: scpr surabaya -d
    #[arg(short, long)]
    pub default: bool,

    /// specific date, format date is YYYY-MM-DD
    /// Example: scpr surabaya --date 2026-06-12
    #[arg(long)]
    pub date: Option<String>,

    /// Display next pray
    #[arg(short)]
    pub next: bool,
}

pub fn maybe_page_output(text: &str) {
    let is_tty = atty::is(atty::Stream::Stdout);

    if !is_tty {
        println!("{text}");
        return;
    }

    // Disable di CI
    if std::env::var("CI").is_ok() {
        println!("{text}");
        return;
    }

    let term_height = terminal_size()
        .map(|(_, Height(h))| h as usize)
        .unwrap_or(24);

    let line_count = text.lines().count();

    if line_count <= term_height {
        println!("{text}");
        return;
    }

    // Respect user PAGER
    if std::env::var("PAGER").is_err() {
        // Cek apakah less tersedia
        if which::which("less").is_ok() {
            unsafe {
                std::env::set_var("PAGER", "less -R");
            }
        } else {
            println!("{text}");
            return;
        }
    }

    pager::Pager::new().setup();
    println!("{text}");
}