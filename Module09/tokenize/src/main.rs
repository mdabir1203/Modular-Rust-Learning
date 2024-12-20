use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: String,
    pub start: usize,
    pub end: usize,
}

pub struct AdvancedTokenizer {
    token_patterns: Vec<(String, Regex)>,
    custom_handlers: HashMap<String, Box<dyn Fn(&str, usize, usize) -> Token>>,
}

impl AdvancedTokenizer {
    pub fn new() -> Self {
        let mut tokenizer = AdvancedTokenizer {
            token_patterns: Vec::new(),
            custom_handlers: HashMap::new(),
        };

        // Default patterns
        tokenizer.add_pattern("WHITESPACE", r"\s+");
        tokenizer.add_pattern("NUMBER", r"\d+(\.\d*)?");
        tokenizer.add_pattern("WORD", r"\w+");
        tokenizer.add_pattern("PUNCTUATION", r"[.,!?;:]");
        tokenizer.add_pattern("OPERATOR", r"[+\-*/=<>]");
        tokenizer.add_pattern("PARENTHESIS", r"[()]");
        tokenizer.add_pattern("BRACKET", r"[\[\]]");
        tokenizer.add_pattern("BRACE", r"[{}]");

        tokenizer
    }

    pub fn add_pattern(&mut self, name: &str, pattern: &str) {
        self.token_patterns.insert(0, (name.to_string(), Regex::new(pattern).unwrap()));
    }

    pub fn add_custom_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str, usize, usize) -> Token + 'static,
    {
        self.custom_handlers.insert(name.to_string(), Box::new(handler));
    }

    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut position = 0;

        while position < text.len() {
            let mut matched = false;

            for (name, pattern) in &self.token_patterns {
                if let Some(m) = pattern.find(&text[position..]) {
                    if m.start() == 0 {
                        let start = position + m.start();
                        let end = position + m.end();
                        let value = m.as_str().to_string();

                        let token = if let Some(handler) = self.custom_handlers.get(name) {
                            handler(&value, start, end)
                        } else {
                            Token {
                                value,
                                token_type: name.clone(),
                                start,
                                end,
                            }
                        };

                        tokens.push(token);
                        position = end;
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                tokens.push(Token {
                    value: text[position..position + 1].to_string(),
                    token_type: "UNKNOWN".to_string(),
                    start: position,
                    end: position + 1,
                });
                position += 1;
            }
        }

        tokens
    }

    pub fn tokenize_with_context(&self, text: &str, context_size: usize) -> Vec<Token> {
        let basic_tokens = self.tokenize(text);
        let mut contextualized_tokens = Vec::new();

        for (i, token) in basic_tokens.iter().enumerate() {
            let context_before = &basic_tokens[i.saturating_sub(context_size)..i];
            let context_after = &basic_tokens[i + 1..std::cmp::min(basic_tokens.len(), i + 1 + context_size)];

            let contextualized_token = self.apply_context_rules(token, context_before, context_after);
            contextualized_tokens.push(contextualized_token);
        }

        contextualized_tokens
    }

    fn apply_context_rules(&self, token: &Token, context_before: &[Token], context_after: &[Token]) -> Token {
        // Example rule: If a number is followed by a word, it might be a unit
        if token.token_type == "NUMBER" && !context_after.is_empty() && context_after[0].token_type == "WORD" {
            return Token {
                value: token.value.clone(),
                token_type: "MEASUREMENT".to_string(),
                start: token.start,
                end: token.end,
            };
        }

        // More context rules can be added here

        token.clone()
    }
}

fn main() {
    let mut tokenizer = AdvancedTokenizer::new();

    // Adding a custom pattern
    tokenizer.add_pattern("EMAIL", r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b");

    // Adding a custom handler
    tokenizer.add_custom_handler("URL", |value, start, end| Token {
        value: value.to_string(),
        token_type: "URL".to_string(),
        start,
        end,
    });
    tokenizer.add_pattern("URL", r"https?://\S+");

    let sample_text = "Let's get it going";
    let tokens = tokenizer.tokenize_with_context(sample_text, 2);

    for token in tokens {
        println!("{}: {} (Position: {}-{})", token.value, token.token_type, token.start, token.end);
    }
}