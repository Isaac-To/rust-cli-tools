use getopts::Options;

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
}
