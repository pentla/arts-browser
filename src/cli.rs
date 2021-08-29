use clap::{AppSettings, Clap};

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "")]
    html: String,
}
