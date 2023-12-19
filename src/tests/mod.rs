use std::path::Path;
use std::thread;
use std::time::Duration;
use crate::jars::jar::Jar;
use crate::jars::manager::JarManager;
use crate::servers::server::Server;
use crate::servers::downloader::Downloader;
use crate::servers::runner::Runner;

#[test]
fn load_jars_includes_all_jars() {
    let manager = JarManager::load().unwrap();
    assert_eq!(manager.jars.len(), 4);
}

#[test]
fn wrong_path_should_return_error() {
    let jar = Jar {
        name: "".to_string(),
        version: None,
        build: None,
    };
    let path = Path::new("wrong_path");
    let server = Server::new(
        "Test Server".to_string(),
        jar,
        path.to_path_buf(),
    );
    assert!(server.is_err());
}

#[test]
fn server_can_start() {
    let jars = JarManager::load().unwrap();
    let jar = &jars.jars[0];
    let jar = jar.get_latest().unwrap();

    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join("servers").join("test_server");

    let mut server = Server::new(
        "Test Server".to_string(),
        jar,
        path.to_path_buf(),
    ).unwrap();
    server.settings.gui = false;

    let downloader = Downloader::new(&server);
    let result = downloader.download();
    assert!(result.is_ok());

    server.accept_eula().unwrap();

    let mut runner = Runner::new(&server, vec![], vec![]);
    let result = runner.start();
    assert!(result.is_ok());

    let mut child = result.unwrap();
    thread::sleep(Duration::from_secs(5));
    let result = child.kill();
    assert!(result.is_ok());
}