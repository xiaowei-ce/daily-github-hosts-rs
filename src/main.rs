use crate::consts::consts::DOMAINMS;
use chrono::Utc;
use rsdns::{
    clients::{ClientConfig, tokio::Client},
    records::{Class, data::A},
};
use std::{
    fs::File,
    io::Write,
    net::SocketAddr,
    str::FromStr,
};
use tokio::task::JoinSet;

pub mod consts;

#[tokio::main]
#[allow(unused)]
async fn main() {
    let mut set = JoinSet::new();
    for domain in DOMAINMS {
        set.spawn(async move { (domain, ip(&domain).await) });
    }

    let mut hosts_file = File::create("hosts").unwrap();
    let mut readme_file = File::create("README.md").unwrap();

    let head = format!(
        "# GitHub IP hosts Start
# Last update: {}
# GitHub URL: https://github.com/xiaowei-ce/daily-github-hosts-rs
",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    );

    writeln!(hosts_file, "{}", head);
    writeln!(readme_file, "```\n{}", head);

    while let Some(res) = set.join_next().await {
        let tupe = res.unwrap();
        if let Some(ips) = tupe.1 {
            ips.iter().for_each(|ip| {
                let formatted = format!("{} {}", tupe.0, ip);
                writeln!(hosts_file, "{}", formatted);
                writeln!(readme_file, "{}", formatted);
            });
        }
    }
    writeln!(hosts_file, "\n# GitHub IP hosts End");

    writeln!(readme_file, "\n# GitHub IP hosts End\n```");

    readme_file.flush().unwrap();
    hosts_file.flush().unwrap();
}

async fn ip(host: &&str) -> Option<Vec<String>> {
    let config = ClientConfig::with_nameserver(SocketAddr::from_str("8.8.8.8:53").unwrap());
    let mut client = Client::new(config).await.unwrap();

    let reslut = client.query_rrset::<A>(*host, Class::IN).await.unwrap();
    let ips = reslut
        .rdata
        .iter()
        .map(|it| it.address.to_string())
        .collect::<Vec<String>>();
    if ips.is_empty() {
        return Option::None;
    }

    Option::Some(ips)
}
