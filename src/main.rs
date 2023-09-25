use reqwest;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;



// Function to perform subnet lookup
async fn get_subnet(subnet: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://bgp.he.net/net/{}#_dns", subnet);

    // Create a custom HTTP client with a User-Agent header
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        // Define a regular expression pattern to match DNS names
        let re = Regex::new(r#"/dns/([^\s"]+)"#).unwrap();

        // Iterate over captured groups (DNS names) and print them
        for cap in re.captures_iter(&body) {
            if let Some(dns_name) = cap.get(1) {
                println!("{}", dns_name.as_str());
            }
        }
    } else {
        eprintln!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

// Function to perform AS number lookup
async fn get_asn(asn: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://bgp.he.net/{}#_prefixes", asn);

    // Create a custom HTTP client with a User-Agent header
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        // Define a regular expression pattern to match subnets
        let re = Regex::new(r#"/net/([\w./]+)"#).unwrap();

        // Iterate over captured groups (subnets)
        for cap in re.captures_iter(&body) {
            if let Some(subnet) = cap.get(1) {
                // Call the get_subnet function for each subnet
                if let Err(err) = get_subnet(subnet.as_str()).await {
                    eprintln!("Failed to fetch data for subnet {}: {:?}", subnet.as_str(), err);
                }
            }
        }
    } else {
        eprintln!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!(r"

     __                       __         __                  __      _
    / /    ____  ____        / / ___    / /_  ___    _____  / /_    (_)  _  __   ___
   / /    / _ / / __ \    __/ / / _ \  / __/ / _ \  / ___/ /  __/  / /  | |/ /  / _ \
  / _ \  /_  / / /_/ /   / _ / /  __/ / /_  /  __/ / /__  /  /_   / /   |   /  /  __/
 /____/ __/ / /  ___/   /___/  \___/  \__/  \___/  \___/  \___/  /_/    |__/   \___/
       /___/ /_/                                                        
 
    ");

    let args: Vec<String> = env::args().collect();

    let mut function = None;
    let mut argument = None;
    let mut list_file = None;

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-l" | "--list" => {
                if i + 1 < args.len() {
                    list_file = Some(&args[i + 1]);
                }
            }
            "subnet" | "asn" => {
                function = Some(arg);
                if i + 1 < args.len() {
                    argument = Some(&args[i + 1]);
                }
            }
            _ => {}
        }
    }

    if function.is_none() || argument.is_none() {
        eprintln!("Usage: bgp.exe <function> <argument> [-l <list_file>]");
        eprintln!("Functions:");
        eprintln!("Subnet: returns hostnames for a given subnet. ");
        eprintln!("ASN: returns hostnames for a given ASN.");
        std::process::exit(1);
    }

    let function = function.unwrap();
    let argument = argument.unwrap();

    match function.as_str() {
        "subnet" => {
            if let Some(list_file) = list_file {
                // Read subnets from the specified file
                let file = File::open(list_file)?;
                let reader = io::BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(subnet) = line {
                        if let Err(err) = get_subnet(subnet.trim()).await {
                            eprintln!("Failed to fetch data for subnet {}: {:?}", subnet, err);
                        }
                    }
                }
            } else {
                // Perform subnet lookup for the provided argument
                get_subnet(argument).await?;
            }
        }
        "asn" => {
            if let Some(list_file) = list_file {
                // Read ASNs from the specified file
                let file = File::open(list_file)?;
                let reader = io::BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(asn) = line {
                        if let Err(err) = get_asn(asn.trim()).await {
                            eprintln!("Failed to fetch data for ASN {}: {:?}", asn, err);
                        }
                    }
                }
            } else {
                // Perform ASN lookup for the provided argument
                get_asn(argument).await?;
            }
        }
        _ => {
            eprintln!("Invalid function. Use 'subnet' or 'asn'.");
            std::process::exit(1);
        }
    }

    Ok(())
}