use std::io::BufRead;

use clap::{command, Parser};
use log::{debug, info, trace};
use simple_logger::SimpleLogger;

#[derive(Debug, Parser)]
#[command(version, about)]
struct ProgramArgs {
    #[arg(short, long, default_value = "input.txt")]
    input_file: String,
    #[arg(short, long, default_value = "output.txt")]
    output_file: String,
}

fn main() -> Result<(), std::io::Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("Failed to init logging");

    let ProgramArgs {
        input_file,
        output_file,
    } = ProgramArgs::parse();

    let mut reader = open_buf_reader(&input_file)?;

    let mut hosts = Vec::with_capacity(10000);

    let mut read_chars_len: usize;

    loop {
        let mut hostsfile_line: String = String::new();
        read_chars_len = reader.read_line(&mut hostsfile_line)?;
        if read_chars_len == 0 {
            info!("Completed reading file {:?}", input_file);
            break;
        } else if read_chars_len == 1 && hostsfile_line.contains("\n") {
            debug!("Empty str")
        } else if hostsfile_line.starts_with("#") {
            debug!("Comment line, ignore");
        } else {
            // println!("Line read - {:?}",stored_line);
            get_host_from_line(&hostsfile_line).map(|host| {
                hosts.push(host);
            });
        }
        hostsfile_line.clear();
    }

    info!("Processed {} hosts", hosts.len());
    debug!("Last item - {:?}", hosts.last());
    write_hosts_to_file(hosts, output_file)?;
    Ok(())
}

fn open_buf_reader(
    input_file: &String,
) -> Result<std::io::BufReader<std::fs::File>, std::io::Error> {
    let file = std::fs::OpenOptions::new().read(true).open(input_file)?;
    let reader = std::io::BufReader::new(file);
    Ok(reader)
}

fn write_hosts_to_file(hosts: Vec<String>, file_name: String) -> Result<(), std::io::Error> {
    std::fs::write(file_name, hosts.join("\n"))
}

fn get_host_from_line(stored_line: &String) -> Option<String> {
    let arrays = stored_line.split(" ");
    let elements: Vec<&str> = arrays.collect();
    if elements.len() != 2 {
        return None;
    }
    let host_string: String = elements[1].trim_end().into();
    trace!("Read host {:?}", host_string);
    Some(host_string)
}
