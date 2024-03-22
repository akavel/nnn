use anyhow::{bail, Context, Result};
use url::Url;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::manaprotocol::callee;

pub struct Handler {
    apps: BTreeMap<PathBuf, Timestamp>,
}

impl Handler {
    pub fn new() -> Result<Handler> {
        query_0install()
    }
}

impl callee::Handler for Handler {
    fn detect(&mut self, path: &Path) -> Result<bool> {
        Ok(self.apps.contains_key(path))
    }

    fn gather(&mut self, path: &Path, shadow_prefix: &Path) -> Result<()> {
        let t = self.apps.get(path).unwrap();
        std::fs::write(shadow_prefix.join(path), format!("{t}"))?;
        Ok(())
    }

    fn affect(&mut self, path: &Path, shadow_prefix: &Path) -> Result<()> {
        bail!("NIY");
    }
}

type Timestamp = u64;

fn query_0install() -> Result<Handler> {
    use std::process::Command;
    // TODO[LATER]: streaming read & parse
    // TODO[LATER]: handle stderr & better handling of errors/failures
    let stdout = Command::new("0install")
        .args(["list-apps", "--xml"])
        .output()?
        //.expect("failed to exec 0install")
        .stdout;
    // FIXME: why std::str::from_utf8(&stdout).unwrap() panicked?
    let s = String::from_utf8_lossy(&stdout);
    //println!("{}", s); //.unwrap());
    let app_list = yaserde::de::from_str::<raw::AppList>(&s).unwrap();
    //println!("{:?}", app_list); //.unwrap());
    let map: Result<BTreeMap<_, _>> = app_list
        .app
        .into_iter()
        .map(|app| {
            let url = Url::parse(&app.interface)?;
            // FIXME: ensure/sanitize/encode safe scheme, no username/password, no ipv6, no port, etc.
            let scheme = url.scheme().to_string();
            let host = url.host_str().unwrap().to_string();
            let path: PathBuf = url
                .path_segments()
                .unwrap()
                .map(|s| Path::new(s).to_path_buf())
                .collect();
            let disk_path = Path::new(&scheme).join(host).join(path);
            Ok((disk_path, app.timestamp))
        })
        .collect();
    //println!("{map:?}");
    Ok(Handler { apps: map? })
}

mod raw {
    use yaserde::YaDeserialize;

    #[derive(YaDeserialize, Debug)]
    #[yaserde(
        rename = "app-list",
        namespace = "http://0install.de/schema/desktop-integration/app-list"
    )]
    pub struct AppList {
        #[yaserde(child)]
        pub app: Vec<App>,
    }

    #[derive(YaDeserialize, Debug)]
    #[yaserde(rename = "app")]
    pub struct App {
        #[yaserde(attribute)]
        pub interface: String,
        #[yaserde(attribute)]
        pub timestamp: u64,
    }
}
