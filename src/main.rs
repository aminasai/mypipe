fn main() {
    println!("Hello, world!");
extern crate clap; 
use clap::App; 
use std::process::Command;

fn main() { 
    let matches = App::new("mypipe")
        .version("1.0")
        .author("Pipe")

        .arg(
            clap::Arg::with_name("in")
            .long("in") 
            .takes_value(true)
            .help("String or fortune")
            .required(true)
        )
        .arg(
            clap::Arg::with_name("out")
            .long("out")
            .takes_value(true)
            .help("Cowsay -f value")
            .required(true)
        )
    .get_matches();  

    if matches.is_present("in") {
        process_func(matches.value_of("in").unwrap().to_string(), matches.value_of("out").unwrap().to_string());
    }
    else{
        println!("Invalid input ! Expected mypipe --in <in> --out <out>");
    }}

fn process_func(firstcmd : String, secondcmd : String){
    let output = 
        Command::new(firstcmd)
                .output()
                .expect("failed to execute process");

    let output_cowsay =
        Command::new(secondcmd)
            .args(&["/C", "Not working on Windows, please run on linux"])
            .arg(String::from_utf8_lossy(&output.stdout).to_string())
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
} 