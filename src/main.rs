use std::io::Error;
use clipboard::{ClipboardContext, ClipboardProvider};
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
  print_version();
  let config = load_configuration().expect("Could not load settings");
  let selected_carpoolers = MultiSelect::new("Select the carpoolers of today:", config.carpoolers)
    .prompt()
    .expect("The selected carpoolers could not be retrieved");

  let report = build_report(selected_carpoolers, config.current_user);
  let formatted_report = format_report(report.clone());
  print_report(&formatted_report);
  copy_report_to_clipboard(&formatted_report);
}

fn print_version() {
  let app_name = env!("CARGO_PKG_NAME");
  let app_version = env!("CARGO_PKG_VERSION");
  
  println!("{} v{}\n", app_name, app_version);
}

fn load_configuration() -> Result<Configuration, Error> {
  match ProjectDirs::from("", "", "RUReady") {
    None => panic!("Could not load settings"),
    Some(proj_dirs) => {
      let config_dir = proj_dirs.config_dir();

      let settings = Config::builder()
          .add_source(config::File::from(config_dir.join("ruready.toml")))
          .build()
          .expect("Could not load settings");

      let carpoolers = settings.get_array("carpoolers").expect("Could not load list of carpoolers");
      let me = settings.get_string("me").expect("Could not load username");

      Ok(Configuration {
        carpoolers: carpoolers
          .iter()
          .map(|v| v.kind.to_string())
          .collect::<Vec<String>>(),
        current_user: me,
      })
    }
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

  carpooler_statuses
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

fn print_report(formatted_report: &String) {
  println!("{}", formatted_report);
}

fn copy_report_to_clipboard(report: &String) {
  let mut ctx: ClipboardContext =
    ClipboardProvider::new().expect("Failed to get clipboard context");
  ctx
    .set_contents(report.to_owned())
    .expect("Failed to set clipboard contents");
  println!("Report copied to clipboard!");
}
