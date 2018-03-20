extern crate rand;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::io::{stdout, Write};
use std::{thread, time};
use rand::Rng;

#[derive(StructOpt)]
#[structopt(name = "bear")]
struct Opt {
    /// How many bears to output
    #[structopt(short = "b", long = "bears", default_value = "1")]
    bears: u64,

    /// Newlines after every <n> bears
    #[structopt(short = "n", long = "newline", default_value = "0")]
    newline: u64,

    /// Randomize newline occurence, overrides and sets <n=1..100>
    #[structopt(short = "r", long = "random")]
    random: bool,

    /// Set delay between bears (ms)
    #[structopt(short = "d", long = "delay", default_value = "0")]
    delay: u64,
}

fn main() {
    let mut rng = rand::thread_rng();
    let opt = Opt::from_args();
    let bears = opt.bears;
    let delay = opt.delay;
    let randomize = opt.random;
    let mut newline = if randomize {
        rng.gen_range(1, 100)
    } else {
        opt.newline
    };
    let ms = time::Duration::from_millis(opt.delay);
    let stdout = stdout();
    let mut stdout = stdout.lock();

    if bears < 1 {
        stdout.write(b"\xF0\x9F\x90\xB1").unwrap();
        stdout.flush().unwrap();
        return;
    }

    let mut n = 1;
    while bears >= n {
        if delay > 0 {
            thread::sleep(ms);
        }
        stdout.write(b"\xF0\x9F\x90\xBB").unwrap();
        if newline > 0 && n % newline == 0 {
            stdout.write(b"\n").unwrap();
            if randomize {
                newline = rng.gen_range(1, 100);
            }
        }
        stdout.flush().unwrap();
        n += 1;
    }
}
