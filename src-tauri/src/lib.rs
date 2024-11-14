use std::{collections::HashMap, env, io::Error};

use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri_plugin_http::reqwest;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Deserialize, Serialize, Debug)]
struct DiscordProperties {
	build_hash: String,
	build_number: u32,
	release_channel: String,
	r#type: String,
	version: String,
}

#[tauri::command]
async fn login(login: String, password: String) -> String {
	let mut map = HashMap::new();
	map.insert("login", &login);
	map.insert("password", &password);

	let error = json!({
		"user_id": "Not a user",
		"mfa": false,
		"sms": false,
		"ticket": "Not a user",
		"backup": false,
		"totp": false,
		"webauthn": null
	});

	let client = reqwest::Client::new();
	let res = client
		.post("https://discord.com/api/v9/auth/login")
		.json(&map)
		.send()
		.await
		.unwrap()
		.json::<Value>()
		.await;
	match res {
		Ok(_) => res.unwrap().to_string(),
		Err(_) => error.to_string(),
	}
}

#[tauri::command]
async fn totp(code: String, ticket: String) -> String {
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
		.json::<Value>()
		.await;
	let error = json!({
		"token": "",
		"user_settings": {
			"locale": "",
			"theme": "",
		},
	});
	if let Ok(res) = res {
		connect_websocket(res["token"].to_string()).await;
		log::info!("yay");
		res.to_string()
	} else {
		log::error!("Error during totp authentification !");
		error.to_string()
	}
}

async fn connect_websocket(token: String) {
	let gateway_url = "wss://gateway.discord.gg/?v=9&encoding=json";

	let (ws_stream, _) = (connect_async(gateway_url).await).unwrap_or_else(|_| {
		log::error!("Problem connecting to discord WebSocket");
		panic!();
	});

	log::info!("Connected to Discord WebSocket");

	let (mut write, mut read) = ws_stream.split();
	let mut os = env::consts::OS;
	if os == "macos" {
		os = "mac";
	}

	let properties = reqwest::Client::new()
		.post(format!("https://cordapi.dolfi.es/api/v2/properties/{os}"))
		.send()
		.await
		.unwrap()
		.json::<Value>()
		.await
		.unwrap();

	let identify_payload = json!({
		"op": 2,
		"d": {
			"token": token,
			"capabilities": 8189,
			"properties": properties,
			"presence": {
				"status": "unknown",
				"since": 0,
				"activities": [],
				"afk": false
			},
			"compress": "false",
			"client_state": {
				"guild_versions": {},
				"highest_last_message_id": "0",
				"read_state_version": "0",
				"user_guild_settings_version": "-1",
				"user_settings_version": "-1",
				"private_channels_version": "0",
				"api_code_version": "0"
			}
		}
	});

	if (write
		.send(Message::Text(identify_payload.to_string()))
		.await)
		.is_err()
	{
		panic!()
	};

	tokio::spawn(async move {
		while let Some(msg) = read.next().await {
			match msg {
				Ok(msg) => {
					if let Message::Text(text) = msg {
						if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&text) {
							handle_discord_event(json_msg).await;
						}
					}
				}
				Err(e) => {
					println!("Error receiving message: {e:?}");
					break;
				}
			}
		}
	});

	// Add heartbeat handling, message responses, etc. here
}

async fn handle_discord_event(event: serde_json::Value) {
	println!("Received event: {:?}", event);
	// Process the event based on the type (e.g., message, presence update)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	rustls::crypto::ring::default_provider()
		.install_default()
		.expect("Failed to install rustls crypto provider");
	if tauri::Builder::default()
		.plugin(tauri_plugin_websocket::init())
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
		.is_err()
	{
		eprintln!("Error Loading tauri");
	}
}
