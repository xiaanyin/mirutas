#[tauri::command]
async fn start_indexing(spec_path: String, code_path: String) -> Result<String, String> {
  println!("Starting indexing process:");
  println!("Spec path: {}", spec_path);
  println!("Code path: {}", code_path);
  
  // TODO: Implement actual indexing logic
  
  // For now just return success message
  Ok("索引创建成功".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_log::Builder::default().build())
    .invoke_handler(tauri::generate_handler![start_indexing])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
