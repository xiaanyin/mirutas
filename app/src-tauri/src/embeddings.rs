use anyhow::Result;
use rand::thread_rng;
use serde::Deserialize;
use std::path::Path;
use std::fs;

// 简单的文本嵌入生成器
pub struct TextEmbedder {
    embedding_dim: usize,
    // 记录是否找到了模型文件
    found_model_files: bool,
    model_dir: Option<String>,
}

// 命令参数结构体
#[derive(Debug, Deserialize)]
pub struct EmbeddingArgs {
    text: String,
    #[serde(rename = "modelPath")]
    model_path: String,
}

impl TextEmbedder {
    // 创建新的嵌入生成器
    pub fn new() -> Result<Self> {
        Ok(Self { 
            embedding_dim: 384,
            found_model_files: false,
            model_dir: None
        })
    }

    // 使用自定义模型路径创建
    pub fn with_model_path(model_path: Option<&str>) -> Result<Self> {
        if let Some(path) = model_path {
            // 检查路径是否存在
            let path = Path::new(path);
            if path.exists() && path.is_dir() {
                println!("使用模型路径: {}", path.display());
                
                // 检查目录中的文件
                let found_config = Path::new(path).join("config.json").exists();
                let found_model = Path::new(path).join("pytorch_model.bin").exists() || 
                                 Path::new(path).join("model.safetensors").exists();
                
                if found_config && found_model {
                    println!("找到模型文件！在实际应用中，这里会加载模型");
                    
                    // 列出目录中的文件，便于调试
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            println!("目录内容:");
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    println!("  - {}", entry.path().display());
                                }
                            }
                        },
                        Err(e) => println!("无法读取目录内容: {}", e)
                    }
                    
                    return Ok(Self { 
                        embedding_dim: 384,
                        found_model_files: true,
                        model_dir: Some(path.to_string_lossy().to_string())
                    });
                } else {
                    println!("目录中未找到必要的模型文件 (config.json, pytorch_model.bin 或 model.safetensors)");
                }
            } else {
                println!("模型路径 {} 不存在或不是目录，使用随机向量", path.display());
            }
        }
        
        // 如果路径不存在或未提供，使用随机向量
        Self::new()
    }

    // 生成文本嵌入
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        println!("生成嵌入，输入文本: {}", text);
        
        if self.found_model_files {
            println!("已找到模型文件，但当前版本只支持随机向量生成");
            println!("在实际应用中，这里会使用模型生成真实的嵌入向量");
            
            if let Some(dir) = &self.model_dir {
                println!("模型目录: {}", dir);
            }
        }
        
        // 生成确定性的随机向量 (基于文本内容)
        // 这确保相同文本总是产生相同的向量
        let text_hash = text.chars().fold(0, |acc, c| acc + (c as u32));
        let seed = text_hash % 10000;
        
        println!("使用文本哈希作为种子: {}", seed);
        
        // 使用简单的哈希作为随机数生成器的种子
        let embedding: Vec<f32> = (0..self.embedding_dim)
            .map(|i| {
                // 使用位置和种子生成伪随机数
                let x = ((i as u32 * 9973 + seed * 6971) % 10000) as f32 / 10000.0;
                x * 2.0 - 1.0  // 转换到 [-1, 1] 范围
            })
            .collect();
        
        Ok(embedding)
    }
}

// 用于命令调用的包装函数
#[tauri::command]
pub async fn get_text_embedding(args: EmbeddingArgs) -> Result<Vec<f32>, String> {
    println!("生成文本嵌入，模型路径: {}", args.model_path);
    
    // 创建嵌入生成器
    let embedder = TextEmbedder::with_model_path(Some(&args.model_path))
        .map_err(|e| format!("无法初始化嵌入模型: {}", e))?;
    
    // 生成嵌入
    let embedding = embedder.embed(&args.text)
        .map_err(|e| format!("生成嵌入失败: {}", e))?;
    
    // 打印信息用于调试
    print_embedding_info(&embedding);
    
    Ok(embedding)
}

// 打印嵌入向量的基本信息
pub fn print_embedding_info(embedding: &[f32]) {
    println!("嵌入向量维度: {}", embedding.len());
    
    if !embedding.is_empty() {
        let first_values: Vec<f32> = embedding.iter().take(5).cloned().collect();
        println!("前5个值: {:?}", first_values);
        
        // 计算一些基本统计数据
        let mut sum = 0.0;
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        
        for &value in embedding.iter() {
            sum += value;
            min = min.min(value);
            max = max.max(value);
        }
        
        let mean = sum / embedding.len() as f32;
        println!("最小值: {:.6}, 最大值: {:.6}, 平均值: {:.6}", min, max, mean);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedder() -> anyhow::Result<()> {
        let embedder = TextEmbedder::with_model_path(Some("path_to_your_model"))?;
        let text = "测试嵌入功能";
        let embedding = embedder.embed(text)?;
        
        // 检查嵌入维度是否正确 (all-MiniLM-L6-v2 输出 384 维向量)
        assert_eq!(embedding.len(), 384);
        
        // 确保向量不全是0
        assert!(embedding.iter().any(|&x| x != 0.0));
        
        println!("嵌入测试通过!");
        Ok(())
    }
}