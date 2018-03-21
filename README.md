# 🐻 bear 🐻
[![Build Status](https://travis-ci.org/hexjelly/bear.svg?branch=master)](https://travis-ci.org/hexjelly/bear) [![Coverage Status](https://coveralls.io/repos/github/hexjelly/bear/badge.svg?branch=master)](https://coveralls.io/github/hexjelly/bear?branch=master)

When you need more bears

## Help
```
USAGE:
    bear.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --random     Randomize newline occurence, overrides and sets <n=1..100>
    -V, --version    Prints version information

OPTIONS:
    -b, --bears <bears>        How many bears to output [default: 1]
    -d, --delay <delay>        Set delay between bears (ms) [default: 0]
    -n, --newline <newline>    Newlines after every <n> bears [default: 0]
```

## Example output
`bear -b 10`:
```
🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻
```

`bear -b 10 -n 3`:
```
🐻🐻🐻
🐻🐻🐻
🐻🐻🐻
🐻
```

`bear -b 100 -r`:
```
🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻
🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻
🐻
🐻
🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻
🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻🐻
🐻🐻🐻🐻🐻🐻
🐻🐻🐻🐻🐻🐻🐻🐻
🐻🐻🐻🐻
```

Let me know if that helps

## License
Apache-2.0

## Contribute
Feel free to create an issue describing any potential bugs or improvements.

Pull requests are welcome, but only if the text contains 🐻 somewhere.
