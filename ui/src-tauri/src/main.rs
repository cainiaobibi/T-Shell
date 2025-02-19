#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::io::{Read, stdout};

use std::process::Command as StdCommand;
use std::{thread};
use std::os::windows::process::CommandExt;
use serde_json::Value::String;
use tauri::async_runtime::handle;
use tauri::{Manager, RunEvent};
use tauri::api::process::{Command as tauriCommand, CommandEvent};

fn main() {
	let mut child = StdCommand::new("server.exe").creation_flags(0x08000000)
		.spawn().expect("cmd exec error!");


/*	let ( rx, child) = tauriCommand::new_sidecar("server")
		.expect("failed to create `my-sidecar` binary command")
		.spawn()
		.expect("Failed to spawn sidecar");
	let mut someChild=Some(child);*/


/*	let mut command = StdCommand::from(
		tauriCommand::new_sidecar("server")
			.expect("Failed to create `backend_server` binary command").spawn()
	);
	let child = command.group_spawn().expect("Failed to spawn backend sidecar");*/



	tauri::Builder::default()
		.plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| { //设置插件
			let window = app.get_window("main").unwrap(); //二次打开软件时，显示已打开窗口，单例运行app
			window.set_focus().unwrap();
			window.show().unwrap();
		}))
		.build(tauri::generate_context!())
		.expect("err building")
		.run(move |handle, event| match event {
			tauri::RunEvent::Ready => {
				println!("程序启动")
			}
			tauri::RunEvent::Exit => {

				//child.kill().expect("!kill");
				//let client = reqwest::Client::new();
				let client = reqwest::blocking::Client::new();
				let res = client.post("http://127.0.0.1:10218/system/shutdown")
					.send();
				println!("{:?}", res);
				//someChild.take().unwrap().kill().expect("!kill");
				child.kill().expect("!kill");
				println!("程序退出")
			}
			_ => ()
		});
}




