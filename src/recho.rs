use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("n", "", "do not output the trailing newline");
    opts.optflag("e", "", "enable interpretation of backslash escapes");
    opts.optflag(
        "E",
        "",
        "disable interpretation of backslash escapes (default)",
    );
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f.to_string());
            std::process::exit(1);
        }
    };

    if matches.opt_present("help") {
        print!(
            r#"
Usage: /bin/recho [SHORT-OPTION]... [STRING]...
  or:  /bin/recho LONG-OPTION
Echo the STRING(s) to standard output.

  -n             do not output the trailing newline
  -e             enable interpretation of backslash escapes
  -E             disable interpretation of backslash escapes (default)
      --help     display this help and exit
      --version  output version information and exit

If -e is in effect, the following sequences are recognized:

  \\      backslash
  \a      alert (BEL)
  \b      backspace
  \c      produce no further output
  \e      escape
  \f      form feed
  \n      new line
  \r      carriage return
  \t      horizontal tab
  \v      vertical tab
  \0NNN   byte with octal value NNN (1 to 3 digits)
  \xHH    byte with hexadecimal value HH (1 to 2 digits)
"#
        );
        return;
    } else if matches.opt_present("version") {
        println!("recho 1.0.0");
        return;
    }

    // Check if it is a single string or multiple strings
    if matches.free.len() == 1 {
        // Single string
        let mut s = matches.free[0].clone();
        if matches.opt_present("e") {
            // Interpret backslash escapes
            let mut new_s = String::new();
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => {
                        if let Some(c) = chars.next() {
                            match c {
                                '\\' => new_s.push('\\'),
                                'a' => new_s.push('\x07'),
                                'b' => new_s.push('\x08'),
                                'c' => break,
                                'e' => new_s.push('\x1B'),
                                'f' => new_s.push('\x0C'),
                                'n' => new_s.push('\n'),
                                'r' => new_s.push('\r'),
                                't' => new_s.push('\t'),
                                'v' => new_s.push('\x0B'),
                                'x' => {
                                    let mut hex = String::new();
                                    for _ in 0..2 {
                                        if let Some(c) = chars.next() {
                                            hex.push(c);
                                        }
                                    }
                                    let byte = u8::from_str_radix(&hex, 16).unwrap();
                                    new_s.push(byte as char);
                                }
                                '0' => {
                                    let mut octal = String::new();
                                    for _ in 0..3 {
                                        if let Some(c) = chars.next() {
                                            if c.is_digit(8) {
                                                octal.push(c);
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                    let byte = u8::from_str_radix(&octal, 8).unwrap();
                                    new_s.push(byte as char);
                                }
                                _ => {
                                    new_s.push('\\');
                                    new_s.push(c)
                                }
                            }
                        }
                    }
                    _ => new_s.push(c),
                }
            }
            s = new_s;
            print!("{}", s);
        } else {
            // raw (default and -E)
            print!("{}", matches.free[0]);
        }
    } else {
        // Multiple strings
        for (i, s) in matches.free.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", s);
        }
    }
}
