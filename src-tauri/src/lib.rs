use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Deserialize, Serialize, Debug)]
struct Login {
	user_id: String,
	mfa: bool,
	sms: bool,
	ticket: String,
	backup: bool,
	totp: bool,
	webauthn: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct UserSettings {
	locale: String,
	theme: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Totp {
	token: String,
	user_settings: UserSettings,
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
		.json::<Login>()
		.await
		.unwrap();
	let ticket = res.ticket;
	log::info!("ticket = {}", ticket);
	Some(ticket)
}

#[tauri::command]
async fn totp(code: String, ticket: String) -> Option<String> {
	let mut map = HashMap::new();
	map.insert("code", &code);
	map.insert("ticket", &ticket);

	let client = reqwest::Client::new();
	let res = client
		.post("https://discord.com/api/v9/auth/mfa/totp")
		.json(&map)
		.send()
		.await
		.unwrap()
		.json::<Totp>()
		.await
		.unwrap();
	let token = res.token;
	log::info!("token = {}", token);
	Some(token)
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
		.invoke_handler(tauri::generate_handler![login, totp])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
