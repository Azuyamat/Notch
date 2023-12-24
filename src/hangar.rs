use serde::Deserialize;
use crate::error::Error;

pub fn fetch_plugins(query: &str) -> Result<Vec<HangarProject>, Error> {
    let url = format!("https://hangar.papermc.io/api/v1/projects?limit=10&offset=0&q={}&sort=-stars", query);
    let response = reqwest::blocking::get(url)?;
    let json: HangarResponse = response.json()?;
    let projects = json.result;
    Ok(projects)
}

#[derive(Deserialize)]
pub struct HangarResponse {
    pub result: Vec<HangarProject>,
}

#[derive(Deserialize)]
pub struct HangarProject {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub name: String,
    pub namespace: HangarNamespace,
    pub stats: HangarStats,
    pub category: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub visibility: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct HangarNamespace {
    pub owner: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct HangarStats {
    pub views: u32,
    pub downloads: u32,
    #[serde(rename = "recentViews")]
    pub recent_views: u32,
    #[serde(rename = "recentDownloads")]
    pub recent_downloads: u32,
    pub stars: u32,
    pub watchers: u32,
}

const ORDER : [&str; 3] = ["Release", "Snapshot", "Alpha"];

pub fn fetch_plugin_info(author: &str, slug: &str) -> Result<Vec<HangarPlugin>, Error> {
    let url = format!("https://hangar.papermc.io/api/v1/projects/{}/{}/versions", author, slug);
    let response = reqwest::blocking::get(url)?;
    let plugin: HangarPluginResponse = response.json()?;
    let mut plugin = plugin.result;
    plugin.sort_by(|a, b| {
        let a = ORDER.iter().position(|&r| r == a.channel.name).unwrap_or(0);
        let b = ORDER.iter().position(|&r| r == b.channel.name).unwrap_or(0);
        a.cmp(&b)
    });

    Ok(plugin)
}

#[derive(Deserialize)]
pub struct HangarPluginResponse {
    result: Vec<HangarPlugin>
}

#[derive(Deserialize)]
pub struct HangarPlugin {
    pub channel: HangarChannel,
    pub downloads: HangarDownloads,
}

#[derive(Deserialize)]
pub struct HangarChannel {
    pub name: String
}

#[derive(Deserialize)]
pub struct HangarDownloads {
    #[serde(rename = "PAPER")]
    pub paper: Option<HangarDownloadsInfo>,
    #[serde(rename = "WATERFALL")]
    pub waterfall: Option<HangarDownloadsInfo>,
    #[serde(rename = "VELOCITY")]
    pub velocity: Option<HangarDownloadsInfo>,
}

#[derive(Deserialize)]
pub struct HangarDownloadsInfo {
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
}