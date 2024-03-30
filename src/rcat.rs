use getopts::Options;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let _program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("A", "show-all", "equivalent to -vET");
    opts.optflag(
        "b",
        "number-nonblank",
        "number nonempty output lines, overrides -n",
    );
    opts.optflag("e", "", "equivalent to -vE");
    opts.optflag("E", "show-ends", "display $ at end of each line");
    opts.optflag("n", "number", "number all output lines");
    opts.optflag("s", "squeeze-blank", "suppress repeated empty output lines");
    opts.optflag("t", "", "equivalent to -vT");
    opts.optflag("T", "show-tabs", "display TAB characters as ^I");
    opts.optflag("u", "", "(ignored)");
    opts.optflag(
        "v",
        "show-nonprinting",
        "use ^ and M- notaion, except for LFD and TAB",
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
            r#"Usage: rcat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help     display this help and exit
      --version  output version information and exit
"#
        );
        return;
    } else if matches.opt_present("version") {
        println!("rcat 1.0.0");
        return;
    }

    let mut v_arg = matches.opt_present("show-nonprinting");
    let mut e_arg = matches.opt_present("show-ends");
    let mut t_arg = matches.opt_present("show-tabs");
    if matches.opt_present("A") {
        v_arg = true;
        e_arg = true;
        t_arg = true;
    }
    if matches.opt_present("e") {
        v_arg = true;
        e_arg = true;
    }
    if matches.opt_present("t") {
        v_arg = true;
        t_arg = true;
    }

    let mut files = matches.free.iter();
    while files.len() > 0 {
        let filename = files.next().unwrap();
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("rcat: {}: {}", filename, e.to_string());
                exit(1);
            }
        };

        let reader = BufReader::new(file);
        let mut count = 0;
        let mut last_line_empty = false;

        for (i, line) in reader.lines().enumerate() {
            let mut line = line.unwrap();
            if t_arg {
                line = line.replace("\t", "^I");
            }
            if matches.opt_present("squeeze-blank") && last_line_empty && line == "" {
                continue;
            } else {
                count += 1;
                last_line_empty = line == "";
            }
            if matches.opt_present("number")
                || (matches.opt_present("number-nonblank") && line == "")
            {
                print!("{:6}\t", count);
            }
            if matches.opt_present("show-all") || v_arg {
                let mut nline = String::new();
                for c in line.chars() {
                    if c == '\t' {
                        continue;
                    }
                    let mut cval = c as u8;
                    if cval >= 128 {
                        cval -= 128;
                        nline.push_str("M-");
                        if cval < 64 {
                            nline.push_str("BM-");
                        } else {
                            cval -= 64;
                            nline.push_str("CM-");
                        }
                    }
                    if cval < 32 || cval == 127 {
                        nline.push('^');
                        cval ^= 0x40;
                    }
                    nline.push(cval as char);
                }
                line = nline;
            }
            print!("{}", line);
            if e_arg {
                print!("$");
            }
            println!();
        }
    }
}
