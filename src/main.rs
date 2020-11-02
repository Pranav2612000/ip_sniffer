use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: sniff <flags> <ip-address>\nflags: -j <num_of_threads> : specify number of threads to use\n -h or --help : Display help");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("invalid IP address")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Invalid number of threads")
                };
                return Ok(Arguments {flag, threads, ipaddr});
            } else {
                return Err("invalid syntax. Run sniff -h for help");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );
}
