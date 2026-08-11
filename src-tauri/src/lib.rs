mod corrector;
use corrector::{CorrectorService, TextStyle};

#[tauri::command]
async fn process_text(text: String, style: TextStyle, api_key: String) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("O texto fornecido não pode estar vazio.".into());
    }

    let service = CorrectorService::new();
    let prompt = service.build_prompt(&text, &style);

    // Exibe o prompt formatado no terminal do Ubuntu para acompanharmos a montagem
    println!("--- Prompt Gerado pelo Rust ---\n{}", prompt);
    println!(
        "Chave de API recebida: {}",
        if api_key.is_empty() {
            "Nenhuma"
        } else {
            "Presente"
        }
    );

    // Retorno temporário (Mock) para testar a ponte IPC antes de plugar a chamada HTTP real
    Ok(format!("[Processado no tom {:?}]: {}", style, text))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_text])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação tauri");
}
