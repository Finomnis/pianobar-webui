use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(short, long, default_value = "12384")]
    pub event_port: u16,
}
