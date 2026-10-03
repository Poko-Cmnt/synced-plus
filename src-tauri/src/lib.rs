#[tauri::command]
async fn save_lrc_android(
  app: tauri::AppHandle,
  file_name: String,
  contents: String,
) -> Result<bool, String> {
  #[cfg(target_os = "android")]
  {
    use tauri_plugin_android_fs::AndroidFsExt;

    let api = app.android_fs_async();
    let uri = api
      .file_picker()
      .save_file(None, &file_name, Some("text/plain"), false)
      .await
      .map_err(|e| e.to_string())?;

    let Some(uri) = uri else {
      return Ok(false);
    };

    if let Err(e) = api.write(&uri, contents.as_bytes()).await {
      let _ = api.remove_file(&uri).await;
      return Err(e.to_string());
    }

    Ok(true)
  }

  #[cfg(not(target_os = "android"))]
  {
    let _ = (app, file_name, contents);
    Err("Android LRC export is only available on Android".to_string())
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save_lrc_android])
    .plugin(tauri_plugin_fs::init());

  #[cfg(target_os = "android")]
  let builder = builder.plugin(tauri_plugin_android_fs::init());

  let builder = builder
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
