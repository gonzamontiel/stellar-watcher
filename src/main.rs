use structopt::StructOpt;
mod stellar_watch;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    address: Option<String>,
    #[structopt(short, long)]
    watch: bool
}

fn main() {
    let args: Cli = Cli::from_args();
    stellar_watch::start(args.address, args.watch)
}
