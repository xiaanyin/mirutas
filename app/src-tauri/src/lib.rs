mod embeddings;

#[tauri::command]
async fn start_indexing(spec_path: String, code_path: String) -> Result<String, String> {
  println!("インデックス作成プロセスを開始:");
  println!("仕様書パス: {}", spec_path);
  println!("コードパス: {}", code_path);
  
  // TODO: 実際のインデックス作成ロジックを実装
  
  // 現時点では成功メッセージを返すだけ
  Ok("インデックス作成成功".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_log::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      start_indexing, 
      embeddings::get_text_embedding
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
