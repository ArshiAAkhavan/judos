use judos::pipeline::Pipeline;
use log::info;
use env_logger;
fn main() {
    env_logger::init();
    info!("rendering config files");
    let config_file = std::fs::File::open("./config.yml").unwrap();
    let p: Pipeline = serde_yaml::from_reader(config_file).unwrap();
    info!("running...");
    p.run();
}
