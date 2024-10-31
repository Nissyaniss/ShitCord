use std::collections::HashMap;

use serde::Deserialize;
use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Deserialize)]
struct Test {
	user_id: String,
	mfa: bool,
	sms: bool,
	ticket: String,
	backup: bool,
	totp: bool,
	webauthn: Option<String>,
}

#[tauri::command]
async fn login(login: String, password: String) -> Option<String> {
	let mut map = HashMap::new();
	map.insert("login", &login);
	map.insert("password", &password);

	let client = reqwest::Client::new();
	let res = client
		.post("https://discord.com/api/v9/auth/login")
		.json(&map)
		.send()
		.await
		.unwrap()
		.json::<Test>()
		.await
		.unwrap();
	let yay = res.user_id;
	log::info!("user_id = {}", yay);
	Some(yay)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_log::Builder::new()
				.target(tauri_plugin_log::Target::new(
					tauri_plugin_log::TargetKind::Stdout,
				))
				.build(),
		)
		.plugin(tauri_plugin_http::init())
		.plugin(tauri_plugin_shell::init())
		.invoke_handler(tauri::generate_handler![login])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
