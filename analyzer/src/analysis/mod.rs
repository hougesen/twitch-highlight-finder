pub fn tokenize_message(
    message: &str,
    emote_scores: &dashmap::ReadOnlyView<String, u8>,
) -> Vec<(String, u8)> {
    let mut tokens = Vec::new();

    for word in message.split_whitespace() {
        if !word.is_empty() && !word.starts_with('@') {
            tokens.push((word.to_string(), *emote_scores.get(word).unwrap_or(&0u8)));
        }
    }

    tokens
}

pub fn calculate_message_score(tokens: &Vec<(String, u8)>) -> u8 {
    let mut modifier = 0;

    for (_token, score) in tokens {
        if *score > modifier {
            modifier = *score;
        }
    }

    100 + modifier
}

pub struct AnalysedMessage {
    pub message: String,
    pub tokens: Vec<(String, u8)>,
    pub message_score: u8,
}

pub fn analyze_message(
    message: String,
    emote_scores: &dashmap::ReadOnlyView<String, u8>,
) -> AnalysedMessage {
    let tokens = tokenize_message(&message, emote_scores);

    AnalysedMessage {
        message,
        message_score: calculate_message_score(&tokens),
        tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_message_score;
    use super::tokenize_message;

    #[test]
    fn test_tokenize_message() {
        use dashmap::DashMap;

        let emote_scores = DashMap::new();

        emote_scores.insert("Kappa".to_string(), 8u8);
        emote_scores.insert("PogChamp".to_string(), 10u8);

        let locked_map = emote_scores.into_read_only();

        // Should set non emotes to 0
        assert_eq!(
            vec![
                ("string".to_string(), 0),
                ("without".to_string(), 0),
                ("emote".to_string(), 0),
            ],
            tokenize_message("   string    without    emote   ", &locked_map)
        );

        // should ignore mentions (@)
        assert_eq!(
            vec![("hello".to_owned(), 0)],
            tokenize_message("@hougesen hello", &locked_map)
        );

        // should return score if word is an emote
        assert_eq!(
            vec![
                ("Kappa".to_string(), 8),
                ("Kappa".to_string(), 8),
                ("Kappa".to_string(), 8)
            ],
            tokenize_message("Kappa Kappa Kappa ", &locked_map)
        );

        assert_eq!(
            vec![("PogChamp".to_string(), 10), ("Kappa".to_string(), 8)],
            tokenize_message("PogChamp  Kappa ", &locked_map)
        );
    }

    #[test]
    fn test_calculate_message_score() {
        assert_eq!(
            100,
            calculate_message_score(&vec![("without".to_string(), 0)])
        )
    }
}
