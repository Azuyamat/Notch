use serde::Deserialize;

/// The response from the API
#[derive(Deserialize)]
pub struct JarVersions {
    pub project: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version_groups: Option<Vec<String>>,
    pub versions: Vec<String>,
}

/// The response from Purpur API for builds
#[derive(Deserialize)]
pub struct JarBuildsPurpur {
    pub project: String,
    pub version: String,
    pub builds: PurpurBuildsInfo,
}

/// The sub-response from Purpur API for builds (latest & all)
#[derive(Deserialize)]
pub struct PurpurBuildsInfo {
    pub latest: String,
    pub all: Vec<String>,
}

/// The response from PaperMC API for builds
#[derive(Deserialize)]
pub struct JarBuildsPaper {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<u32>,
}