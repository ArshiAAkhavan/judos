use judos::pipeline::Pipeline;
fn main() {
    let config_file = std::fs::File::open("./config.yml").unwrap();
    let p: Pipeline = serde_yaml::from_reader(config_file).unwrap();
    println!("{p:?}");
}
