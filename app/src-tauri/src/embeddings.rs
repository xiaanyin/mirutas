use anyhow::Result;
use rand::thread_rng;
use serde::Deserialize;
use std::path::Path;
use std::fs;

// シンプルなテキスト埋め込み生成器
pub struct TextEmbedder {
    embedding_dim: usize,
    // モデルファイルが見つかったかどうかを記録
    found_model_files: bool,
    model_dir: Option<String>,
}

// コマンド引数構造体
#[derive(Debug, Deserialize)]
pub struct EmbeddingArgs {
    text: String,
    #[serde(rename = "modelPath")]
    model_path: String,
}

impl TextEmbedder {
    // 新しい埋め込み生成器を作成
    pub fn new() -> Result<Self> {
        Ok(Self { 
            embedding_dim: 384,
            found_model_files: false,
            model_dir: None
        })
    }

    // カスタムモデルパスで作成
    pub fn with_model_path(model_path: Option<&str>) -> Result<Self> {
        if let Some(path) = model_path {
            // パスが存在するかチェック
            let path = Path::new(path);
            if path.exists() && path.is_dir() {
                println!("モデルパスを使用: {}", path.display());
                
                // ディレクトリ内のファイルをチェック
                let found_config = Path::new(path).join("config.json").exists();
                let found_model = Path::new(path).join("pytorch_model.bin").exists() || 
                                 Path::new(path).join("model.safetensors").exists();
                
                if found_config && found_model {
                    println!("モデルファイルが見つかりました！実際のアプリケーションでは、ここでモデルをロードします");
                    
                    // デバッグ用にディレクトリ内のファイルを一覧表示
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            println!("ディレクトリの内容:");
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    println!("  - {}", entry.path().display());
                                }
                            }
                        },
                        Err(e) => println!("ディレクトリの内容を読み取れません: {}", e)
                    }
                    
                    return Ok(Self { 
                        embedding_dim: 384,
                        found_model_files: true,
                        model_dir: Some(path.to_string_lossy().to_string())
                    });
                } else {
                    println!("ディレクトリに必要なモデルファイル (config.json, pytorch_model.bin または model.safetensors) が見つかりません");
                }
            } else {
                println!("モデルパス {} は存在しないか、ディレクトリではありません。ランダムベクトルを使用します", path.display());
            }
        }
        
        // パスが存在しないか、提供されていない場合は、ランダムベクトルを使用
        Self::new()
    }

    // テキスト埋め込みを生成
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        println!("埋め込みを生成、入力テキスト: {}", text);
        
        if self.found_model_files {
            println!("モデルファイルが見つかりましたが、現在のバージョンではランダムベクトル生成のみサポートしています");
            println!("実際のアプリケーションでは、ここでモデルを使用して実際の埋め込みベクトルを生成します");
            
            if let Some(dir) = &self.model_dir {
                println!("モデルディレクトリ: {}", dir);
            }
        }
        
        // テキスト内容に基づいて決定論的なランダムベクトルを生成
        // これにより同じテキストが常に同じベクトルを生成することを保証
        let text_hash = text.chars().fold(0, |acc, c| acc + (c as u32));
        let seed = text_hash % 10000;
        
        println!("テキストハッシュをシードとして使用: {}", seed);
        
        // シンプルなハッシュを乱数生成器のシードとして使用
        let embedding: Vec<f32> = (0..self.embedding_dim)
            .map(|i| {
                // 位置とシードを使用して疑似乱数を生成
                let x = ((i as u32 * 9973 + seed * 6971) % 10000) as f32 / 10000.0;
                x * 2.0 - 1.0  // [-1, 1] 範囲に変換
            })
            .collect();
        
        Ok(embedding)
    }
}

// コマンド呼び出し用のラッパー関数
#[tauri::command]
pub async fn get_text_embedding(args: EmbeddingArgs) -> Result<Vec<f32>, String> {
    println!("テキスト埋め込みを生成、モデルパス: {}", args.model_path);
    
    // 埋め込み生成器を作成
    let embedder = TextEmbedder::with_model_path(Some(&args.model_path))
        .map_err(|e| format!("埋め込みモデルを初期化できません: {}", e))?;
    
    // 埋め込みを生成
    let embedding = embedder.embed(&args.text)
        .map_err(|e| format!("埋め込み生成に失敗しました: {}", e))?;
    
    // デバッグ用に情報を出力
    print_embedding_info(&embedding);
    
    Ok(embedding)
}

// 埋め込みベクトルの基本情報を出力
pub fn print_embedding_info(embedding: &[f32]) {
    println!("埋め込みベクトルの次元: {}", embedding.len());
    
    if !embedding.is_empty() {
        let first_values: Vec<f32> = embedding.iter().take(5).cloned().collect();
        println!("最初の5つの値: {:?}", first_values);
        
        // 基本的な統計データを計算
        let mut sum = 0.0;
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        
        for &value in embedding.iter() {
            sum += value;
            min = min.min(value);
            max = max.max(value);
        }
        
        let mean = sum / embedding.len() as f32;
        println!("最小値: {:.6}, 最大値: {:.6}, 平均値: {:.6}", min, max, mean);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedder() -> anyhow::Result<()> {
        let embedder = TextEmbedder::with_model_path(Some("path_to_your_model"))?;
        let text = "埋め込み機能をテスト";
        let embedding = embedder.embed(text)?;
        
        // 埋め込み次元が正しいかチェック (all-MiniLM-L6-v2 は 384 次元ベクトルを出力)
        assert_eq!(embedding.len(), 384);
        
        // ベクトルがすべて0ではないことを確認
        assert!(embedding.iter().any(|&x| x != 0.0));
        
        println!("埋め込みテスト合格!");
        Ok(())
    }
}