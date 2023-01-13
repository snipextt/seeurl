use std::{collections::HashMap, env, io::Write};

use anyhow::Result;
use reqwest::blocking::Client;

use crate::utils::{
    client::{build_request, parse_request_options, parse_response},
    help::print_help,
};

pub fn read_args() -> Result<()> {
    let mut options_map = HashMap::<&str, &str>::new();
    let args: Vec<String> = env::args().skip(1).collect();
    let help = args.iter().any(|arg| arg == "-h" || arg == "--help") | (args.len() == 0);
    if help {
        print_help();
        return Ok(());
    }
    let ver = args.iter().any(|arg| arg == "-v" || arg == "--version");
    if ver {
        println!("seeurl v0.1.0");
        return Ok(());
    }
    args.iter().enumerate().for_each(|(i, arg)| {
        if (i + 1 >= args.len()) && arg.starts_with("-") {
            panic!("Error: Missing value for option {}", arg);
        }
        match arg.as_str() {
            "-v" | "--verbose" => options_map.insert("verbose", "true"),
            "-H" | "--headers" => options_map.insert("headers", &args[i + 1]),
            "-o" | "--output" => options_map.insert("output", &args[i + 1]),
            "-t" | "--timeout" => options_map.insert("timeout", &args[i + 1]),
            "-m" | "--method" => options_map.insert("method", &args[i + 1]),
            "-p" | "--params" => options_map.insert("params", &args[i + 1]),
            "-b" | "--body" => options_map.insert("body", &args[i + 1]),
            "-c" | "--cookie" => options_map.insert("cookie", &args[i + 1]),
            "-d" | "--download" => options_map.insert("download", "true"),
            v if !v.starts_with("-") && (i % 2 == 0) && (i != args.len() - 1) => {
                panic!("Error: Invalid option {}", arg)
            }
            _ => None,
        };
    });
    let url = args.last().expect("Url is required");
    options_map.insert("url", url);
    let options = parse_request_options(options_map)?;
    let request = build_request(&options)?;
    let client = Client::new();
    let response = client.execute(request)?;

    let out_file = options.output;
    let parsed_res = parse_response(response)?;
    if out_file != "" {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(out_file.clone())?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(parsed_res.as_bytes())?;
    } else {
        println!("{}", parsed_res);
    }
    Ok(())
}
