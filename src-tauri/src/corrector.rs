use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub enum TextStyle {
    Grammar,
    Professional,
    Polite,
    Casual,
    Shorter,
    Detailed,
}

impl TextStyle {
    pub fn instruction(&self) -> &'static str {
        match self {
            TextStyle::Grammar => "Corrija apenas erros ortográficos e gramaticais do texto. Mantenha o idioma original.",
            TextStyle::Professional => "Reescreva o texto adotando um tom estritamente profissional, claro e corporativo.",
            TextStyle::Polite => "Reescreva o texto de forma extremamente educada, gentil e cordial.",
            TextStyle::Casual => "Reescreva o texto de forma despojada, natural e informal.",
            TextStyle::Shorter => "Reescreva o texto resumindo, trazendo os pontos necessarios.",
            TextStyle::Detailed => "Reescreva o texto de forma Detalhada, Deixando claro os pontos e o que deve ser feito.",
        }
    }
}

pub struct CorrectorService;

impl CorrectorService{
    pub fn new()-> Self {
        Self
    }

    ///Prompt Final pra LLM
    pub fn build_prompt(&self, text: &str, style: &TextStyle) -> String {
        format!(
            "Instrução: {}\nRegra Estrita: Retorne APENAS o texto processado final, sem saudações ou explicações.\n\nTexto Original:\n{}",
            style.instruction(),
            text.trim()
        )
    }


}