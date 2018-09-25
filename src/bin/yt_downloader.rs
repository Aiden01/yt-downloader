extern crate clap;
extern crate yt_downloader;
extern crate colored;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use yt_downloader::{is_logged};
use yt_downloader::youtube::search_result::Video;

use colored::*;
use std::io;


fn build_cli() -> App<'static, 'static> {
    App::new("yt-downloader")
        .version("1.0.0")
        .author("WebD")
        .about("Download youtube videos using the command line")
        .arg(Arg::with_name("search")
            .short("s")
            .long("search")
            .value_name("SEARCH_QUERY")
            .takes_value(true))
        .arg(Arg::with_name("max-results")
            .short("m")
            .long("max")
            .value_name("MAX_RESULTS")
            .default_value("20")
            .takes_value(true))
}

fn main() {
    let args: ArgMatches = build_cli().get_matches();
    if let Some(search) = args.value_of("search") {
        if is_logged() {
            println!("{}", &format!("Searching for {}...\n", search).yellow());
            match yt_downloader::youtube::search(search.to_string(), args.value_of("max-results").unwrap().to_string()) {
                Some(search_results) => {
                    for result in &search_results {
                        let format = &format!("{} - {}\n\nChannel: {}", result.id, &result.title.blue(), &result.channelTitle.green());
                        println!("{}\n", format);
                    }
                    ask_for_install(search_results);
                },
                None => println!("{}", &format!("No result for {}", search.underline()).red())
            }
        } else {
            println!("{}", "You must be logged to use this command! See yt-download login --help".red())
        }
    }
}

fn ask_for_install(search_results: Vec<Video>) {
    println!("{}", "Please enter the ID of the videos that you want to install. Ex: [1 9 14]".underline());
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let numbers = input.trim().split_whitespace();
            for id in numbers {
                match id.parse::<i32>() {
                    Ok(parsed_id) => {
                        if let Some(that) = search_results.clone().into_iter().find(|video| video.id == parsed_id) {
                            println!("{}", &format!("\n===> Installing {}...", that.title.underline()).yellow().italic());
                            match that.download() {
                                Ok(_) => println!("{}", "\nInstallation completed.".green()),
                                Err(_) => println!("{}", "Error when downloading the video.".red())
                            }
                        } else {
                            println!("{}", &format!("Cannot find the package #{}", parsed_id).red());
                            std::process::exit(0);
                        }
                    },
                    Err(_) => println!("{}", "Please enter valid numbers.".red())
                }
            }
        },
        Err(e) => println!("{}", &e.to_string().red())
    }
}

