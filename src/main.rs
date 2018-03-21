extern crate rand;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::io::{stdout, Write};
use std::{thread, time};
use rand::Rng;

#[derive(StructOpt, Default, Debug, PartialEq)]
#[structopt(name = "bear")]
struct Opt {
    /// How many bears to output
    #[structopt(short = "b", long = "bears", default_value = "1")]
    pub bears: u64,
    /// Newlines after every <n> bears
    #[structopt(short = "n", long = "newline", default_value = "0")]
    pub newline: u64,
    /// Randomize newline occurence, overrides and sets <n=1..100>
    #[structopt(short = "r", long = "random")]
    pub random: bool,
    /// Set delay between bears (ms)
    #[structopt(short = "d", long = "delay", default_value = "0")]
    pub delay: u64,
}

fn main() {
    let opt = Opt::from_args();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    bear(&mut stdout, &opt);
}

fn bear<W: Write>(buf: &mut W, opt: &Opt) {
    let bears = opt.bears;
    if bears < 1 {
        buf.write(b"\xF0\x9F\x90\xB1").unwrap();
        buf.flush().unwrap();
        return;
    }

    let ms = time::Duration::from_millis(opt.delay);
    let mut rng = rand::thread_rng();
    let mut newline = if opt.random {
        rng.gen_range(1, 100)
    } else {
        opt.newline
    };

    let mut n = 1;
    while bears >= n {
        if opt.delay > 0 {
            thread::sleep(ms);
        }
        buf.write(b"\xF0\x9F\x90\xBB").unwrap();
        if newline > 0 && n % newline == 0 {
            buf.write(b"\n").unwrap();
            if opt.random {
                newline = rng.gen_range(1, 100);
            }
        }
        buf.flush().unwrap();
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_bears() {
        let mut buf = vec![];
        let opt = Opt::default();
        bear(&mut buf, &opt);
        assert_eq!(&[240, 159, 144, 177], &buf[..]);
    }
}
