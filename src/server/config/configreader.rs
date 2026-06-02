use crate::server::config::config::Config;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::MutexGuard;

pub fn load_config_file(mut config_singelton: MutexGuard<Config>) {
    if let Ok(lines) = read_lines("C:\\Miasql\\config.mcon") {
        for line in lines.map_while(Result::ok) {
            let mut splits = line.trim().split("=");
            let mut ident = splits.next().unwrap_or_default().to_uppercase();
            ident = ident.trim().to_string();

            if (ident == "CONFIG VERSION") {
                match Some(splits.next()) {
                    Some(config) => {
                        config_singelton.config_version =
                            config.unwrap().trim().parse::<f32>().unwrap_or_default();
                        println!("config version: {:?}", config_singelton.config_version);
                    }
                    None => println!("config version: missing"),
                }
            } else if (ident == "MIA VERSION") {
                match Some(splits.next()) {
                    Some(mia) => {
                        config_singelton.mia_version = mia.unwrap().trim().to_string();
                        println!("mia version: {:?}", config_singelton.mia_version);
                    }
                    None => println!("config version: missing"),
                }
            } else if (ident == "TYPE") {
                match Some(splits.next()) {
                    Some(license) => {
                        config_singelton.licence_type = license.unwrap().trim().to_string();
                        println!("license: {:?}", config_singelton.licence_type);
                    }
                    None => println!("config version: missing"),
                }
            } else if (ident == "MASTERQUEUE SIZE") {
                match Some(splits.next()) {
                    Some(capacity) => {
                        config_singelton.masterqueue_capacity =
                            capacity.unwrap().trim().parse::<u32>().unwrap_or_default();
                        println!("masterqueue: {:?}", config_singelton.masterqueue_capacity);
                    }
                    None => println!("config version: missing"),
                }
            }else if (ident == "LEDGER LOCATION") {
                match Some(splits.next()) {
                    Some(location) => {
                        config_singelton.ledger_location = location.unwrap().trim().to_string();
                        println!("ledger location: {:?}", config_singelton.ledger_location);
                    }
                    None => println!("config version: missing"),
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
