use clap::Parser;
mod xcap;
mod region;
mod winmac_region;

fn main() {
    let args = Args::parse();

    if args.fullscreen {
        xcap::fullscreen_shot();
    }
    else if args.area {
        xcap::region_screenshot();
    }
    else if args.test {
        winmac_region::test();
    }
    else {
        println!("Invalid flag")
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    area: bool,

    #[arg(long)]
    fullscreen: bool,

    #[arg(long)]
    test: bool,
}
