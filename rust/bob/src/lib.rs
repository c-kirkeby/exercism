pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if is_yelling(m) && is_question(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_saying_nothing(m) => "Fine. Be that way!",
        _ => "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.to_lowercase() != message
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}

fn is_saying_nothing(message: &str) -> bool {
    message == ""
}
