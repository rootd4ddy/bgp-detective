Key Features:

- Subdomain enumeration for specific subnets or ASNs.
- Utilizes Hurricane Electric's BGP Toolkit as a data source.
- Rust-based for high performance and reliability.


Discover hidden subdomains and gain valuable insights into network topology with BGP Detective. Enhance your cybersecurity and reconnaissance capabilities today.

## Installation

To use BGP Detective, you'll need Rust installed on your system. Follow these steps to get started:


1. Clone the repository: 
```git clone https://github.com/yourusername/bgp-detective.git && cd bgp-detective```

2. Build the project. 
```cargo build --release```

An ELF binary or Executable will be built in src/target/release

Examples

Return hostnames for a given subnet:
```bgp.exe subnet 203.0.113.0/24```

Return hostnames for a given ASN. 
```bgp.exe asn AS745```

