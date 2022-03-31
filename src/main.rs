use crossbeam::channel;
use env_logger;
use judos::pipeline::Pipeline;
use log::info;
fn main() {
    env_logger::init();
    info!("rendering config files");
    let config_file = std::fs::File::open("./config.yml").unwrap();
    let p: Pipeline = serde_yaml::from_reader(config_file).unwrap();
    info!("running...");
    let (stx, srx) = channel::bounded(0);
    ctrlc::set_handler(move || {
        stx.send(())
            .expect("unable to send sigkill, you might as well panic!");
    })
    .expect("unable to set SIGINT handler");
    // println!("{p:#?}");
    p.run(srx);
}
