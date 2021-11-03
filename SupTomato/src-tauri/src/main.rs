#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  // #[cfg(target_os = "macos")]
  // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

  app.run(|_, _| {});
}
