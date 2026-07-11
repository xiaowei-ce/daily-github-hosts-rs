use rsdns::{
    clients::{ClientConfig, tokio::Client},
    records::{Class, data::A},
};
use std::{fs::OpenOptions, io::Write, net::SocketAddr, str::FromStr};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let domains = vec![
        "alive.github.com",
        "api.github.com",
        // "actions.githubusercontent.com",
        "api.individual.githubcopilot.com",
        "api.mcp.github.com",
        "avatars.githubusercontent.com",
        "avatars0.githubusercontent.com",
        "avatars1.githubusercontent.com",
        "avatars2.githubusercontent.com",
        "broker.actions.githubusercontent.com",
        "camo.githubusercontent.com",
        "central.github.com",
        "cloud.githubusercontent.com",
        "codeload.github.com",
        "collector.github.com",
        "containers.pkg.github.com",
        "copilot-proxy.githubusercontent.com",
        "copilot-reports.github.com",
        "desktop.githubusercontent.com",
        "dependabot-actions.githubapp.com",
        "default.exp-tas.com",
        "docker.pkg.github.com",
        "docker-proxy.pkg.github.com",
        "education.github.com",
        "favicons.githubusercontent.com",
        "fulcio.githubapp.com",
        "ghcr.io",
        "gist.github.com",
        "github-cloud.s3.amazonaws.com",
        "github-com.s3.amazonaws.com",
        "github.blog",
        "github.com",
        "github.community",
        "github.dev",
        "github.global.ssl.fastly.net",
        "github.io",
        "github.map.fastly.net",
        //  "githubassets.com",
        "githubcopilot.com",
        //  "githubusercontent.com",
        "githubstatus.com",
        "github.githubassets.com",
        "github-cloud.githubusercontent.com",
        "github-cloud.s3.amazonaws.com",
        "github-releases.githubusercontent.com",
        "github-registry-files.githubusercontent.com",
        "gcr.io",
        "launch.actions.githubusercontent.com",
        "live.github.com",
        "lfs.github.com",
        "maven.pkg.github.com",
        "media.githubusercontent.com",
        //  "msecnd.net",
        "mpsghub.actions.githubusercontent.com",
        "npm.pkg.github.com",
        "npm-proxy.pkg.github.com",
        "npm-beta-proxy.pkg.github.com",
        "npm-beta.pkg.github.com",
        "npmregistryv2prod.blob.core.windows.net",
        "nuget.pkg.github.com",
        "objects.githubusercontent.com",
        "objects-origin.githubusercontent.com",
        "origin-tracker.githubusercontent.com",
        "pipelines.actions.githubusercontent.com",
        "pipelinesghubeus1.actions.githubusercontent.com",
        "pkg.github.com",
        "pkg.actions.githubusercontent.com",
        "pkg-containers.githubusercontent.com",
        "productionresultssa1.blob.core.windows.net",
        "pypi.pkg.github.com",
        "raw.githubusercontent.com",
        "raw.github.com",
        "release-assets.githubusercontent.com",
        "results-receiver.actions.githubusercontent.com",
        "rubygems.pkg.github.com",
        "runnerghubeus1.actions.githubusercontent.com",
        "run-actions-1-azure-eastus.actions.githubusercontent.com",
        "runner-auth.actions.githubusercontent.com",
        "scanning-api.github.com",
        "setup-tools.actions.githubusercontent.com",
        "swift.pkg.github.com",
        "timestamp.githubapp.com",
        "tokenghub.actions.githubusercontent.com",
        "tuf-repo.github.com",
        "user-images.githubusercontent.com",
        "uploads.github.com",
        "visualstudio.com",
        "vscode.dev",
        // "vscode-webview.net",
        "vstoken.actions.githubusercontent.com",
    ];

    let mut set = JoinSet::new();
    for domain in domains {
        set.spawn(async move { (domain, ip(&domain).await) });
    }

    let mut hosts_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("hosts")
        .unwrap();

    while let Some(res) = set.join_next().await {
        let tupe = res.unwrap();
        if let Some(ips) = tupe.1 {
            ips.iter().for_each(|ip| {
                hosts_file
                    .write_all(format!("{} {}\n", tupe.0, ip).as_bytes())
                    .unwrap();
            });
        }
    }
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
