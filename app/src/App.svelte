<script>
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  
  let specPath = '';
  let codePath = '';
  let modelPath = '';
  let textInput = '';
  let embedding = null;
  let isLoading = false;
  let isEmbeddingLoading = false;
  let errorMessage = '';

  async function selectSpecFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择式样书文件夹'
      });
      if (selected) {
        specPath = selected;
      }
    } catch (error) {
      console.error('选择式样书文件夹出错:', error);
      errorMessage = `选择文件夹出错: ${error.message}`;
    }
  }

  async function selectCodeFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择代码文件夹'
      });
      if (selected) {
        codePath = selected;
      }
    } catch (error) {
      console.error('选择代码文件夹出错:', error);
      errorMessage = `选择文件夹出错: ${error.message}`;
    }
  }

  async function selectModelFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择模型文件夹'
      });
      if (selected) {
        modelPath = selected;
      }
    } catch (error) {
      console.error('选择模型文件夹出错:', error);
      errorMessage = `选择文件夹出错: ${error.message}`;
    }
  }

  async function startProcess() {
    if (!specPath || !codePath) {
      errorMessage = "请先选择两个文件夹";
      return;
    }

    isLoading = true;
    errorMessage = '';
    
    try {
      // 这里将来会添加实际的处理逻辑
      await invoke('start_indexing', { specPath, codePath });
      // 成功后跳转到下一页
      // window.location.href = '/viewer';
    } catch (error) {
      console.error('处理出错:', error);
      errorMessage = `处理出错: ${error.message}`;
    } finally {
      isLoading = false;
    }
  }

  async function generateEmbedding() {
    if (!textInput) {
      errorMessage = "请输入要生成嵌入的文本";
      return;
    }

    isEmbeddingLoading = true;
    errorMessage = '';
    
    try {
      // 如果没有选择模型路径，使用一个默认路径或空字符串
      const path = modelPath || "./models";
      
      // 将参数作为单个对象传递，与EmbeddingArgs结构体匹配
      embedding = await invoke('get_text_embedding', { 
        args: {
          text: textInput, 
          modelPath: path
        }
      });
      
      console.log('生成的嵌入向量:', embedding);
    } catch (error) {
      console.error('生成嵌入出错:', error);
      errorMessage = `生成嵌入出错: ${error.message || error}`;
      embedding = null;
    } finally {
      isEmbeddingLoading = false;
    }
  }

  function resetPaths() {
    specPath = '';
    codePath = '';
    modelPath = '';
    textInput = '';
    embedding = null;
    errorMessage = '';
  }
</script>

<main>
  <h1>Mirutas - 项目导航</h1>
  
  <div class="container">
    <div class="path-section">
      <h2>式样书文件夹</h2>
      <div class="path-display">
        <p class="path">{specPath || '未选择'}</p>
        <button on:click={selectSpecFolder}>浏览...</button>
      </div>
    </div>

    <div class="path-section">
      <h2>代码文件夹</h2>
      <div class="path-display">
        <p class="path">{codePath || '未选择'}</p>
        <button on:click={selectCodeFolder}>浏览...</button>
      </div>
    </div>

    <div class="path-section">
      <h2>模型文件夹</h2>
      <div class="path-display">
        <p class="path">{modelPath || '未选择'}</p>
        <button on:click={selectModelFolder}>浏览...</button>
      </div>
    </div>

    <div class="embedding-test">
      <h2>嵌入测试</h2>
      <textarea 
        bind:value={textInput} 
        placeholder="输入要生成嵌入的文本"
        rows="3"
      ></textarea>
      <button 
        on:click={generateEmbedding} 
        disabled={!textInput || isEmbeddingLoading}
        class="primary-button"
      >
        {isEmbeddingLoading ? '生成中...' : '生成嵌入'}
      </button>

      {#if embedding}
        <div class="embedding-result">
          <h3>嵌入结果 (前10个值):</h3>
          <p class="embedding-data">
            {embedding.slice(0, 10).map(v => v.toFixed(4)).join(', ')}...
          </p>
          <p>向量维度: {embedding.length}</p>
        </div>
      {/if}
    </div>

    {#if errorMessage}
      <div class="error-message">
        {errorMessage}
      </div>
    {/if}

    <div class="actions">
      <button on:click={resetPaths}>重置</button>
      <button 
        on:click={startProcess} 
        disabled={!specPath || !codePath || isLoading}
        class="primary-button"
      >
        {isLoading ? '处理中...' : '开始索引'}
      </button>
    </div>
  </div>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  h1 {
    color: #1a1a1a;
    font-size: 2.5rem;
    margin-bottom: 2rem;
  }

  h2 {
    font-size: 1.2rem;
    margin-bottom: 0.5rem;
  }

  h3 {
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }

  .container {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .path-section {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .path-display {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
  }

  .path {
    flex: 1;
    padding: 0.5rem;
    background-color: #f5f5f5;
    border-radius: 4px;
    border: 1px solid #ddd;
    min-height: 1.5rem;
    word-break: break-all;
  }

  .actions {
    display: flex;
    justify-content: space-between;
    margin-top: 1rem;
  }

  button {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    border: 1px solid #ddd;
    background-color: #f5f5f5;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }

  button:hover:not(:disabled) {
    background-color: #e5e5e5;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .primary-button {
    background-color: #4f46e5;
    color: white;
    border: none;
  }

  .primary-button:hover:not(:disabled) {
    background-color: #4338ca;
  }

  .error-message {
    color: #dc2626;
    background-color: #fee2e2;
    padding: 0.5rem;
    border-radius: 4px;
    margin-top: 0.5rem;
  }

  textarea {
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
    border: 1px solid #ddd;
    margin-bottom: 1rem;
    font-family: inherit;
  }

  .embedding-test {
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 1rem;
    background-color: #f9fafb;
    border-radius: 4px;
    border: 1px solid #e5e7eb;
  }

  .embedding-result {
    margin-top: 1rem;
    padding: 1rem;
    background-color: #f0f9ff;
    border-radius: 4px;
    border: 1px solid #bae6fd;
  }

  .embedding-data {
    font-family: monospace;
    word-break: break-all;
    background-color: #ffffff;
    padding: 0.5rem;
    border-radius: 4px;
    overflow-x: auto;
  }
</style>
