use config::Config;
use directories::ProjectDirs;
use inquire::MultiSelect;

struct Configuration {
  carpoolers: Vec<String>,
  current_user: String,
}

#[derive(Clone)]
struct CarpoolerStatus {
  carpooler: String,
  ready: bool,
}

fn main() {
  let config = load_configuration();
  let selected_carpoolers =
    MultiSelect::new("Select the carpoolers of today:", config.carpoolers).prompt();

  match selected_carpoolers {
    Ok(_) => {
      let report = build_report(selected_carpoolers.unwrap(), config.current_user);
      let formatted_report = format_report(report.clone());
      print_report(formatted_report);
    }
    Err(_) => panic!("The selected carpoolers could not be retrieved"),
  }
}

fn load_configuration() -> Configuration {
  if let Some(proj_dirs) = ProjectDirs::from("", "", "RUReady") {
    let config_dir = proj_dirs.config_dir();

    let settings = Config::builder()
      .add_source(config::File::from(config_dir.join("ruready.toml")))
      .build()
      .unwrap();

    let carpoolers = settings.get_array("carpoolers");
    if carpoolers.is_err() {
      panic!("Could not load list of carpoolers")
    }

    let me = settings.get_string("me");
    if me.is_err() {
      panic!("Could not load username")
    }

    return Configuration {
      carpoolers: carpoolers
        .unwrap()
        .iter()
        .map(|v| v.kind.to_string())
        .collect::<Vec<String>>(),
      current_user: me.unwrap(),
    };
  } else {
    panic!("Could not load settings")
  }
}

fn build_report(
  selected_carpoolers: Vec<String>,
  current_carpooler: String,
) -> Vec<CarpoolerStatus> {
  let mut carpoolers = selected_carpoolers.clone();
  carpoolers.push(current_carpooler.clone());
  let carpooler_statuses = carpoolers
    .iter()
    .map(|c| CarpoolerStatus {
      carpooler: c.to_string(),
      ready: c.eq(current_carpooler.as_str()),
    })
    .collect();

  return carpooler_statuses;
}

fn format_report(report: Vec<CarpoolerStatus>) -> String {
  sort_report(report)
      .iter()
      .map(|cs| format!("{} {}", if cs.ready { "✔" } else { "❌" }, cs.carpooler))
      .collect::<Vec<String>>()
      .join("\n")
}

fn sort_report(report: Vec<CarpoolerStatus>) -> Vec<CarpoolerStatus> {
  let mut sorted_report = report.clone();
  sorted_report.sort_by(|a, b| a.carpooler.cmp(&b.carpooler));
  sorted_report
}

fn print_report(formatted_report: String) {
  println!("{}", formatted_report);
}
