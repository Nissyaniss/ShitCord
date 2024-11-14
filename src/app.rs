#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use dioxus_router::prelude::{Link, Routable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
	async fn invoke_args(cmd: &str, args: JsValue) -> JsValue;

	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
	async fn invoke(cmd: &str) -> JsValue;
}

#[derive(Clone, Routable, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},
	#[route("/login")]
	Login {},
}

#[derive(Serialize, Deserialize)]
struct LoginArgs {
	login: String,
	password: String,
}

#[derive(Serialize, Deserialize)]
struct TotpArgs {
	code: String,
	ticket: String,
}

#[derive(Serialize, Deserialize)]
struct ConnectWebsocketArgs {
	token: String,
}

#[derive(Serialize, Deserialize)]
struct TotpJson {
	token: String,
	user_settings: UserSettings,
}

#[derive(Serialize, Deserialize)]
struct UserSettings {
	locale: String,
	theme: String,
}

#[derive(Serialize, Deserialize)]
struct LoginJson {
	user_id: String,
	mfa: bool,
	sms: bool,
	ticket: String,
	backup: bool,
	totp: bool,
	webauthn: Option<String>,
}

#[derive(Clone)]
struct User {
	email: Signal<String>,
	password: Signal<String>,
	code: Signal<String>,
	login_json: Signal<LoginJson>,
	totp_json: Signal<TotpJson>,
}

#[component]
pub fn Home() -> Element {
	rsx! {
		link { rel: "stylesheet", href: "styles.css" }
		Link { to: Route::Login {}, "Go to login" }
	}
}

#[component]
pub fn Login() -> Element {
	let mut user = use_context_provider(|| User {
		email: Signal::new(String::new()),
		password: Signal::new(String::new()),
		code: Signal::new(String::new()),
		login_json: Signal::new(LoginJson {
			user_id: String::new(),
			mfa: false,
			sms: false,
			ticket: String::new(),
			backup: false,
			totp: false,
			webauthn: Some(String::new()),
		}),
		totp_json: Signal::new(TotpJson {
			token: String::new(),
			user_settings: UserSettings {
				locale: String::new(),
				theme: String::new(),
			},
		}),
	});

	// let mut totp = use_context_provider(|| Totp {
	// 	code: Signal::new(String::new()),
	// });

	rsx! {
		link { rel: "stylesheet", href: "styles.css" }
		main {
			class: "container",
			if user.login_json.read().ticket == *"Not a user" || user.login_json.read().ticket.is_empty() {
				form {
					class: "row",
					onsubmit: login,
					input {
					   class: "login-input",
					   placeholder: "Enter a email...",
					   value: "{user.email}",
					   r#type: "email",
					   oninput: move |event| user.email.set(event.value())
					},
					input {
					   class: "login-input",
					   placeholder: "Enter a password...",
					   value: "{user.password}",
					   r#type: "password",
					   oninput: move |event| user.password.set(event.value())

					},
					button { r#type: "submit", "Login" },
				}
			},
			if user.login_json.read().ticket == *"Not a user" {
				p {
					class: "error",
					"Email or password incorrect."
				}
			}
			if user.login_json.read().ticket != *"Not a user" && !user.login_json.read().ticket.is_empty() && user.login_json.read().totp == true {
				form {
					class: "row",
					onsubmit: totp,
					input {
						id: "login-input",
						placeholder: "Enter un code",
						value: "{user.code}",
						oninput: move |event| user.code.set(event.value())
					},
					button { r#type: "submit", "Confirmer" },
				}
			}
		}
	}
}

async fn login(_form_event: FormEvent) {
	let mut user = use_context::<User>();

	if user.email.read().is_empty() || user.password.read().is_empty() {
		return;
	}

	let login_args = serde_wasm_bindgen::to_value(&LoginArgs {
		login: user.email.read().to_string(),
		password: user.password.read().to_string(),
	})
	.unwrap();
	user.login_json.set(
		serde_json::from_str(
			invoke_args("login", login_args)
				.await
				.as_string()
				.unwrap()
				.as_str(),
		)
		.unwrap(),
	);
}

async fn totp(_form_event: FormEvent) {
	let mut user = use_context::<User>();

	if user.code.read().is_empty() {
		return;
	}

	let totp_args = serde_wasm_bindgen::to_value(&TotpArgs {
		code: user.code.read().to_string(),
		ticket: user.login_json.read().ticket.to_string(),
	})
	.unwrap();
	user.totp_json.set(
		serde_json::from_str(
			invoke_args("totp", totp_args)
				.await
				.as_string()
				.unwrap()
				.as_str(),
		)
		.unwrap(),
	);
}
