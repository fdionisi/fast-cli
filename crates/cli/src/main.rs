use std::fmt;

use colored::*;
use fast_cli_core::{Fast, Info};
use futures::StreamExt;
use spinners_rs::{Spinner, Spinners};

const DONWLOAD_ARROW: &'static str = "↓";
const UPLOAD_ARROW: &'static str = "↑";
const SEPARATOR: &'static str = "/";

fn display_load(
    load_unit: &String,
    load: &Option<String>,
    done: bool,
    arrow: &'static str,
) -> impl fmt::Display {
    let unit = if load_unit.len() == 0 {
        "Mbps".into()
    } else {
        load_unit.clone()
    };

    let temp = format!(
        "{} {} {}",
        load.as_ref().unwrap_or(&"-".into()),
        unit.dimmed(),
        arrow
    );

    if done {
        temp.green()
    } else {
        temp.blue()
    }
}

struct Message(Option<Info>);

impl Message {
    fn new() -> Self {
        Message(None)
    }

    fn update(&mut self, info: Info) {
        self.0.replace(info);
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: String = match self.0 {
            Some(ref info) => {
                format!(
                    "{} {} {}",
                    display_load(
                        &info.download_unit,
                        &info.download_speed,
                        info.is_download_done,
                        DONWLOAD_ARROW,
                    ),
                    SEPARATOR.dimmed(),
                    display_load(
                        &info.upload_unit,
                        &info.upload_speed,
                        info.is_upload_done,
                        UPLOAD_ARROW,
                    )
                )
            }
            None => format!(
                "{} {} {}",
                display_load(&"Mbps".to_string(), &None, false, DONWLOAD_ARROW,),
                SEPARATOR.dimmed(),
                display_load(&"Mbps".to_string(), &None, false, UPLOAD_ARROW,)
            ),
        };

        write!(f, "{}", display)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fast = Fast::new()?;
    let mut spinner: Spinner = Spinners::Dots.into();

    let mut message = Message::new();

    spinner.set_message(&message);
    spinner.start();

    while let Some(ref info) = fast.next().await {
        message.update(info.clone());

        if !info.is_done() {
            spinner.set_message(&message);
        } else {
            spinner.stop_with_message(format!("  {}\n", &message));
        }
    }

    Ok(())
}
