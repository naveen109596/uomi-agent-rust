use wasm_bindgen::prelude::*;

// Simple UOMI Agent (WASM)
#[wasm_bindgen]
pub struct UomiAgent {
    name: String,
}

#[wasm_bindgen]
impl UomiAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> UomiAgent {
        UomiAgent { name }
    }

    pub fn greet(&self, user: String) -> String {
        format!("👋 Hello {}, I am {}. I am a UOMI Agent running in WASM!", user, self.name)
    }

    pub fn answer(&self, question: String) -> String {
        let q = question.to_lowercase();

        if q.contains("role") {
            "🎖️ UOMI has roles like Supporter, Elite, Inner Circle, and Expert.".to_string()
        } else if q.contains("token") {
            "💰 The $UOMI token will launch via IDOs on CV PAD, Seedify, and Poolz.".to_string()
        } else if q.contains("agent") {
            "🤖 You can deploy agents on https://app.uomi.ai/agents (needs 100 UOMI).".to_string()
        } else if q.contains("validator") {
            "🔒 Validators run nodes in the UOMI network and earn rewards.".to_string()
        } else {
            "🤔 I don’t know that yet, but I’m learning every day!".to_string()
        }
    }
}
