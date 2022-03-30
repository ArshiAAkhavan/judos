use judos::pipeline::Pipeline;
use log::info;
use env_logger;
use crossbeam::channel;
fn main() {
    env_logger::init();
    info!("rendering config files");
    let config_file = std::fs::File::open("./config.yml").unwrap();
    let p: Pipeline = serde_yaml::from_reader(config_file).unwrap();
    info!("running...");
    let (stx,srx) = channel::bounded(0);
    ctrlc::set_handler(move ||{
        stx.send(()).expect("unable to send sigkill, you might as well panic!");
    });
    p.run(srx);
}
