use const_format::formatcp as const_format;
use pianobar_webserver::default_config;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(short, long, default_value = const_format!("{}", default_config::EVENT_PORT))]
    pub event_port: u16,

    #[structopt(short, long, default_value = const_format!("{}", default_config::WEBSERVER_PORT))]
    pub port: u16,

    #[structopt(short, long, help = "The path to the build directory of the web ui")]
    pub webpage_folder: Option<String>,

    #[structopt(
        short,
        long,
        parse(from_occurrences),
        help = "Increases verbosity. Can be used multiple times."
    )]
    pub verbose: usize,

    #[structopt(
        long,
        help = "Changes the pianobar executable path. For custom pianobar installations that are not in PATH.",
        default_value = default_config::PIANOBAR_COMMAND
    )]
    pub pianobar_path: String,

    #[structopt(
        long,
        help = "Specifies the path of the pianobar config file",
        default_value = "~/.config/pianobar/config"
    )]
    pub pianobar_config: String,
}
