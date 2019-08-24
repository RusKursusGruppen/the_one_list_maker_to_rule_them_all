use std::env;
use std::fs::{File, remove_file};
use std::io::prelude::*;
use std::io::{self, Read};
use std::process::Command;
use std::str::from_utf8;

fn beer_header<'a>() -> &'a [u8] {
    r"\documentclass{article}
\usepackage[a3paper, hmargin={1.5cm, 1.5cm}, vmargin={2.5cm, 2.5cm}]{geometry}
\usepackage[utf8]{inputenc}
\usepackage[table]{xcolor}
\pagenumbering{gobble}

\begin{document}
\def\arraystretch{2.5}
".as_bytes()
}

fn footer<'a>() -> &'a [u8] {
    r"\end{document}".as_bytes()
}

fn gigs_header<'a>() -> &'a [u8] {
    r"\documentclass{article}
\usepackage[a3paper, hmargin={1.5cm, 1.5cm}, vmargin={2.5cm, 2.5cm}]{geometry}
\usepackage[utf8]{inputenc}
\usepackage[table]{xcolor}

\begin{document}
\def\arraystretch{10}
".as_bytes()
}

fn beer_table_header<'a>() -> &'a[u8] {
    r"\begin{tabular}{|p{6cm}|p{7cm}|p{4cm}|p{4cm}|p{2.5cm}|}
\hline \huge{\textbf{Navn}} & \huge{\textbf{Øl}} &
\huge{\textbf{Cider}} & \huge{\textbf{Sodavand}} &
\huge{\textbf{Cocio}}\\\hline
".as_bytes()
}

fn gigs_table_header<'a>(days: &[String]) -> String {
    let width = 24usize;
    let mut s: String = r"\begin{tabular}{|".to_string();
    s.push_str(&format!("p{{{}cm}}|", (width / (1 + days.len()))));
    for _d in days.iter() {
        s.push_str(&format!("p{{{}cm}}|", (width / (1 + days.len()))));
    }
    s.push_str("}\\hline\n~");
    for d in days.iter() {
        s.push_str(&format!(r"& \huge{{{}}} ", d));
    }
    s.push_str("\\\\\\hline\n");
    s
}

fn table_footer<'a>() -> &'a [u8] {
    r"\end{tabular}
".as_bytes()
}

fn help(file: &str) {
    println!("Usage: {0} [--gigs] [--beer] [--beer-stdin]\n\
              Example: {0} --gigs < data.csv\n\
              Example: {0} --gigs-stdin < gigs.csv\n\
              Example: {0} --beer",
             file);
}

fn is_gigs(s: &[String]) -> bool {
    s.contains(&"--gigs".to_string()) || s.contains(&"-g".to_string())
}

fn is_beer(s: &[String]) -> bool {
    s.contains(&"--beer".to_string()) || s.contains(&"-b".to_string())
}

fn is_gigs_stdin(s: &[String]) -> bool {
    s.contains(&"--gigs-stdin".to_string()) || s.contains(&"-s".to_string())
}

fn compile(f: &str) -> Result<(), std::io::Error> {
    match Command::new("pdflatex")
        .arg(f)
        .output() {
        Ok(output) => {
            remove_file(format!("./{}.tex", f))?;
            remove_file(format!("./{}.aux", f))?;
            remove_file(format!("./{}.log", f))?;
            if output.status.success() {
                println!("Success - output written to {}.pdf", f)
            } else {
                println!("Failed with exit code {:#?}\n{}",
                         output.status.code().unwrap(),
                         from_utf8(&output.stdout).unwrap())
            }

            Ok(())
        }
        Err(e) => Err(e)
    }
}

fn generate_gigs(is_stdin: bool) -> std::io::Result<()> {
    let filename_prelude = "gigs";

    let mut file = File::create(format!("{}.tex", filename_prelude))?;
    file.write(gigs_header())?;


    let gigs = if is_stdin {
        let mut v = vec![];

        let mut data = String::new();
        io::stdin().read_to_string(&mut data)?;
        for l in data.lines() {
            let x: Vec<&str> = l.split(",").collect();
            v.push((x[0].to_string(), x[1..].into_iter().map(|s| if s.trim() == "true" { true } else { false }).collect()));
        }

        v
    } else {
        vec![
            ("Anti-kaos".to_string(), vec![false, true, true]),
            ("Morgenmad".to_string(), vec![false, true, true]),
            ("Frokost".to_string(), vec![false, true, true]),
            ("Oprydning (Morgenmad)".to_string(), vec![false, true, true]),
            ("Oprydning (Frokost)".to_string(), vec![false, true, true]),
            ("Aftensmad".to_string(), vec![true, true, true]),
            ("Oprydning (Aftensmad)".to_string(), vec![true, true, true])
        ]
    };

    file.write(gigs_table_header(&vec!["Tirsdag".to_string(), "Onsdag".to_string(), "Torsdag".to_string()]).as_bytes())?;
    for (gig, days) in gigs.iter() {
        file.write(format!(r"\huge{{{}}} & ", gig).as_bytes())?;
        for (i, day) in days.iter().enumerate() {
            file.write(format!("{} ", if *day { r"\cellcolor{white}" } else { r"\cellcolor{black}" }).as_bytes())?;
            if i + 1 < days.len() {
                file.write("& ".as_bytes())?;
            }
        }
        file.write("\\\\\\hline\n".as_bytes())?;
    }

    file.write(table_footer())?;
    file.write(footer())?;

    compile(&filename_prelude)
}

fn generate_beer() -> std::io::Result<()> {
    let filename_prelude = "beer";

    let mut data = String::new();
    io::stdin().read_to_string(&mut data)?;

    let mut file = File::create(format!("{}.tex", filename_prelude))?;
    file.write(beer_header())?;
    file.write(beer_table_header())?;
    for (i, l) in data.lines().enumerate() {
        if i % 2 == 0 {
            file.write(r"\rowcolor{lightgray}".as_bytes())?;
        }
        file.write(
            format!("{} & & & &\\\\\\hline\n", l).as_bytes()
        )?;
        if i % 33 == 32 {
            file.write(table_footer())?;
            file.write("\\newpage\n".as_bytes())?;
            file.write(beer_table_header())?;
        }
    }

    file.write(table_footer())?;
    file.write(footer())?;

    compile(&filename_prelude)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            help(&args[0]);
        }
        2 => {
            if is_gigs(&args[1..]) {
                generate_gigs(false)?;
            } else if is_gigs_stdin(&args[1..]) {
                generate_gigs(true)?;
            } else if is_beer(&args[1..]) {
                generate_beer()?;
            } else {
                help(&args[0]);
            }
        }
        3 => {
            if is_beer(&args[1..]) {
                generate_beer()?;
            } else {
                help(&args[0]);
            }
        }
        _ => help(&args[0])
    }

    Ok(())
}
