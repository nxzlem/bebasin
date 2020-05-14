use pest::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    Error(Box<dyn std::error::Error>),
    IOError(std::io::Error),
    PestRuleError(pest::error::Error<Rule>),
    SerdeJSONError(serde_json::Error),
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    NixError(nix::Error),
    ZipError(zip::result::ZipError),
    String(String),
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::Error(err) => err.to_string(),
            ErrorKind::IOError(err) => err.to_string(),
            ErrorKind::PestRuleError(err) => err.to_string(),
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            ErrorKind::NixError(err) => err.to_string(),
            ErrorKind::SerdeJSONError(err) => err.to_string(),
            ErrorKind::ZipError(err) => err.to_string(),
            ErrorKind::String(err) => err.to_owned(),
        }
    }
}

// TODO
// Migrate to HashSet to remove duplication (?)
type Hosts = HashMap<String, Vec<String>>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HostsParser;

pub fn write_to_file(file_path: &str, hosts: &Hosts, header: &str) -> Result<(), ErrorKind> {
    let mut file;
    match fs::File::create(file_path) {
        Ok(f) => {
            file = f;
        }
        Err(err) => {
            return Err(ErrorKind::IOError(err));
        }
    };

    let mut hosts_stringify = String::new();
    hosts_stringify.push_str(header);
    for host in hosts {
        let ip = &host.0;
        let hostnames = &host.1.join(" ");

        hosts_stringify.push_str(&format!("{} {}\n", ip, hostnames));
    }

    match file.write_all(hosts_stringify.as_bytes().as_ref()) {
        Ok(()) => Ok(()),
        Err(err) => Err(ErrorKind::IOError(err)),
    }
}

pub fn parse_from_file(file_path: &str) -> Result<Hosts, ErrorKind> {
    match fs::read_to_string(file_path) {
        Ok(str) => parse_from_str(&str),
        Err(err) => Err(ErrorKind::IOError(err)),
    }
}

pub fn parse_from_str(str: &str) -> Result<Hosts, ErrorKind> {
    let mut hosts: Hosts = HashMap::new();
    let res = HostsParser::parse(Rule::main, str);

    if let Err(err) = res {
        return Err(ErrorKind::PestRuleError(err));
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
                            }
                            Rule::hostnames => {
                                for hostname in inner_pair.into_inner() {
                                    hostnames.push(hostname.as_str().to_owned());
                                }
                            }
                            _ => {}
                        }
                    }

                    match hosts.get_mut(&ip) {
                        Some(old_val) => {
                            old_val.append(&mut hostnames);
                        }
                        None => {
                            hosts.insert(ip, hostnames);
                        }
                    };
                }
                _ => {}
            }
        }
    }
    Ok(hosts)
}
