use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Name: {}", args.name);
    println!("Fullscreen: {}", args.fullscreen);
    println!("Area: {}", args.area);
}

#[derive(Parser)]
#[command(name = "MyShot Tool")]
struct Args {
    name: String,

    #[arg(long)]
    area: bool,

    #[arg(long)]
    fullscreen: bool,
}