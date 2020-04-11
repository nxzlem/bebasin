use std::fs;
use pest::Parser;
use pest::error::Error;
use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

type Hosts = HashMap<String, Vec<String>>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HostsParser;

pub fn write_to_file(file_path: &str, hosts: &Hosts) -> io::Result<()> {
    let mut file;
    match fs::File::create(file_path) {
        Ok(f) => {
            file = f;
        }
        Err(err) => {
            return Err(err);
        }
    };

    let mut hosts_stringify = String::from(include_str!("../misc/header"));
    for host in hosts {
        let ip = &host.0;
        let hostnames = &host.1.join(" ");

        hosts_stringify.push_str(&format!("{} {}\n", ip, hostnames));
    }

    file.write_all(hosts_stringify.as_bytes().as_ref())
}

pub fn parse_from_file(file_path: &str) -> Result<Hosts, Error<Rule>> {
    let str = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    parse_from_str(&str)
}

pub fn parse_from_str(str: &str) -> Result<Hosts, Error<Rule>> {
    let mut hosts: Hosts = HashMap::new();
    let res = HostsParser::parse(Rule::main, str);

    if let Err(err) = res {
        return Err(err);
    }

    for pairs in res {
        for pair in pairs {
            match pair.as_rule() {
                Rule::statement => {
                    let mut ip = String::new();
                    let mut hostnames: Vec<String> = Vec::new();

                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::ip => {
                                ip = inner_pair.as_str().to_owned();
                            },
                            Rule::hostnames => {
                                for hostname in inner_pair.into_inner() {
                                    hostnames.push(hostname.as_str().to_owned());
                                }
                            },
                            _ => {}
                        }
                    }

                    match hosts.get_mut(&ip) {
                        Some(old_val) => {
                            old_val.append(&mut hostnames);
                        },
                        None => {
                            hosts.insert(ip, hostnames);
                        }
                    };
                },
                _ => {}
            }
        }
    }
    Ok(hosts)
}
