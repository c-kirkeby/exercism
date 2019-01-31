pub fn reply(message: &str) -> &str {
    if is_yelling(message) && is_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_question(message) {
        "Sure."
    } else if is_saying_nothing(message) {
        "Fine. Be that way!"
    } else if is_yelling(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.to_lowercase() != message
}

fn is_question(message: &str) -> bool {
    message.trim().ends_with("?")
}

fn is_saying_nothing(message: &str) -> bool {
    message.trim() == ""
}