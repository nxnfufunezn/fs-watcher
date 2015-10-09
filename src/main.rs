extern crate notify;
extern crate argparse;

use argparse::{ArgumentParser, Store};
use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let mut dir = "".to_string();
    let mut action = "".to_string();
    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Monitor file changes and perform some action");
        argparse.refer(&mut dir)
            .add_option(&["--watch"], Store,
            "Watch the dir for fs changes")
            .required();
        argparse.refer(&mut action)
            .add_option(&["--action"], Store,
            "Action to perform on fs changes")
            .required();
        argparse.parse_args_or_exit();
    }
    env::set_current_dir(&Path::new(&dir)).is_ok();
    let args: Vec<&str> = action.split(' ').collect();
    let (tx, rx) = channel();
    let w: Result<RecommendedWatcher, Error> = Watcher::new(tx);
    match w {
        Ok(mut watcher) => {
            watcher.watch(dir);
            loop {
                match rx.recv() {
                    _ => {
                         let output = Command::new(&args[0])
                                              .args(&args[1..])
                                              .output()
                                              .unwrap_or_else(|e| {
                                                  panic!("Failed to perform action: {}", e)
                                              });
                         println!("{}", String::from_utf8_lossy(&output.stdout));
                         println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
            }
        },
        Err(_) => println!("Error")
    }
}
