use config::Config;
use directories::ProjectDirs;

struct Configuration {
    carpoolers: Vec<String>,
    current_user: String,
}

fn main() {
    let config = load_configuration();
    print_report(config);
}

fn load_configuration() -> Configuration {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "RUReady") {
        let config_dir = proj_dirs.config_dir();

        let settings = Config::builder()
            .add_source(config::File::from(config_dir.join("ruready.toml")))
            .build()
            .unwrap();

        let carpoolers = settings.get_array("carpoolers");
        if carpoolers.is_err() { panic!("Could not load list of carpoolers") }

        let me = settings.get_string("me");
        if me.is_err() { panic!("Could not load username") }

        return Configuration {
            carpoolers: carpoolers.unwrap().iter()
                .map(|v| { v.kind.to_string() })
                .collect::<Vec<String>>(),
            current_user: me.unwrap(),
        };
    } else { panic!("Could not load settings") }
}

fn print_report(config: Configuration) {
    config.carpoolers.iter()
        .map(|c| {
            let status = if c.eq(config.current_user.as_str()) { "✔" } else { "❌" };
            return format!("{} {}", status, c);
        })
        .for_each(|l| { println!("{}", l) });
}
