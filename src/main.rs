use std::env;
use std::collections::HashMap;
use config_injector::process_files;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    let env_vars: HashMap<String, String> = env::vars().collect();
    
    let files_to_process = env::var("FILES_TO_PROCESS")
        .unwrap_or_else(|_| String::from("server-data/server.cfg,txData/default/config.json"));
    let files: Vec<&str> = files_to_process.split(',').collect();
    
    process_files(files, &env_vars)
}
