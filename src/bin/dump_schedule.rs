use std::io::Write;
use bevy::prelude::*;
use bevy_mod_debugdump::schedule_graph;
use untitled_rustgame::build_game;
use std::process::{Command, Stdio};

fn main() {
    const DIRECTORY: &str = "schedules";

    std::fs::remove_dir_all(DIRECTORY).expect("Failed to remove directory");
    std::fs::create_dir_all(DIRECTORY).expect("Could not create schedules folder");

    let mut app = build_game();
    let schedule_labels:Vec<_> = app.world().resource::<Schedules>()
        .iter()
        .map(|(_, schedule)| schedule.label())
        .collect();

    for schedule_label in schedule_labels.iter() {
        let file_path = format!("{DIRECTORY}/{:?}_schedule.png", schedule_label);
        let graph:String = bevy_mod_debugdump::schedule_graph_dot(&mut app, *schedule_label, &schedule_graph::Settings::default());
        let mut command = Command::new("dot")
            .arg("-Tpng")
            .arg(format!("-o{file_path}"))
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        command
            .stdin
            .as_mut()
            .expect("failed to open stdin")
            .write_all(graph.as_bytes())
            .expect("failed to write graph");

        let _output = command
            .wait_with_output()
            .expect("failed to wait for dot");

        info!("Dump Schedule Graph: {file_path}")
    }

}
