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
    if opt.bears < 1 {
        buf.write(b"\xF0\x9F\x90\xB1").unwrap();
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
    while opt.bears >= n {
        if opt.delay > 0 {
            thread::sleep(ms);
            buf.flush().unwrap();
        }
        buf.write(b"\xF0\x9F\x90\xBB").unwrap();
        if newline > 0 && n % newline == 0 {
            buf.write(b"\n").unwrap();
            buf.flush().unwrap();
            if opt.random {
                newline = rng.gen_range(1, 100);
            }
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bears_0() {
        let mut buf = vec![];
        bear(&mut buf, &Opt::default());

        assert_eq!(&[240, 159, 144, 177], &buf[..]);
    }

    #[test]
    fn bears_1() {
        let mut buf = vec![];
        let opt = Opt {
            bears: 1,
            ..Default::default()
        };
        bear(&mut buf, &opt);

        assert_eq!(&[240, 159, 144, 187], &buf[..]);
    }

    #[test]
    fn bears_delay() {
        let mut buf = vec![];
        let opt = Opt {
            bears: 10,
            delay: 500,
            ..Default::default()
        };

        let now = time::Instant::now();
        bear(&mut buf, &opt);

        assert!(now.elapsed().as_secs() >= 5);
    }

    #[test]
    fn bears_random() {
        let mut buf = vec![];
        let opt = Opt {
            bears: 100,
            random: true,
            ..Default::default()
        };
        bear(&mut buf, &opt);

        assert!(buf.iter().filter(|b| **b == 10).count() >= 1);
    }

    #[test]
    fn bears_100() {
        let mut buf = vec![];
        let opt = Opt {
            bears: 100,
            ..Default::default()
        };
        bear(&mut buf, &opt);

        assert_eq!(400, buf.len());
        assert_eq!(&[240, 159, 144, 187], &buf[..4]);
        assert_eq!(&[144, 187, 240, 159], &buf[250..254]);
        assert!(
            buf.iter()
                .all(|&b| b == 240 || b == 159 || b == 144 || b == 187)
        );
    }

    #[test]
    fn bears_100_split_20() {
        let mut buf = vec![];
        let opt = Opt {
            bears: 100,
            newline: 20,
            ..Default::default()
        };
        bear(&mut buf, &opt);

        assert_eq!(405, buf.len());
        assert_eq!(&[240, 159, 144, 187], &buf[4..8]);
        assert_eq!(&[240, 159, 144, 187, 10], &buf[76..81]);
        assert_eq!(&[240, 159, 144, 187, 10], &buf[157..162]);
        assert!(
            buf.iter()
                .all(|&b| b == 240 || b == 159 || b == 144 || b == 187 || b == 10)
        );
    }
}
