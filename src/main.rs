use clap::Parser;
use gcd_cli::gcd;

#[derive(Parser)]
#[command(name = "gcd", about, version)]
struct Args {
    a: i64,
    b: i64,
}

fn main() {
    let args = Args::parse();
    println!("{}", gcd(args.a, args.b));
}
