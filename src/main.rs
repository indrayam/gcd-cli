use clap::Parser;
use gcd_cli::gcd;

#[derive(Parser)]
#[command(
    name = "gcd",
    about = "Print the greatest common divisor of two integers",
    after_help = "Examples:\n  gcd 12 18\n  gcd -- -48 18"
)]
struct Args {
    #[arg(value_name = "A")]
    a: i64,
    #[arg(value_name = "B")]
    b: i64,
}

fn main() {
    let args = Args::parse();
    println!("{}", gcd(args.a, args.b));
}
