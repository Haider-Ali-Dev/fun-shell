use clap::{App, Arg};
use colour::{e_blue, yellow};

fn main() {
    let app =App::new("fun_shell")
        .about("Bored? Fun-Shell makes your console fun.")
        .author("Haider Ali")
        .arg(
            Arg::new("catfact")
            .short('c')
            .long("catfact")
            .takes_value(false)
            .help("Gives a fact about cats.")
        )
        .arg(
            Arg::new("")
        );
    let matches  = app.get_matches();

    let cat_fact = matches.value_of("catfact");

    match cat_fact {
        Some(_) => {
            println!("------------------------");
            let cat = cli::request::request_cat();
            e_blue!("-- Cat Fact --\n");
            yellow!("{}", cat.fact);
            println!("------------------------");
        },
        None => {
            println!("------------------------");
            let cat = cli::request::request_cat();
            e_blue!("-- Cat Fact --\n");
            yellow!("{}\n", cat.fact);
            println!("------------------------");
        }
    }
}
