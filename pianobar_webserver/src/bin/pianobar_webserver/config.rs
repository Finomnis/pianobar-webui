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

    #[structopt(short, long, parse(from_occurrences), help = "Increases verbosity")]
    pub verbose: usize,
}
