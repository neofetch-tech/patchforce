/// Ukraine Independence Day Event Program
/// Celebrates Ukrainian Independence Day on August 24th

use std::fmt;

/// Represents a greeting for Ukrainians
#[derive(Debug, Clone)]
pub struct Greeting {
    message: String,
    language: Language,
}

/// Supported languages for greetings
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Ukrainian,
    English,
    French,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Ukrainian => write!(f, "Українська"),
            Language::English => write!(f, "English"),
            Language::French => write!(f, "Français"),
        }
    }
}

impl Greeting {
    pub fn new(message: String, language: Language) -> Self {
        Greeting { message, language }
    }

    pub fn display(&self) {
        println!("[{}] {}", self.language, self.message);
    }
}

/// Independence Day Event
pub struct IndependenceDayEvent {
    year: u32,
    greetings: Vec<Greeting>,
}

impl IndependenceDayEvent {
    pub fn new(year: u32) -> Self {
        IndependenceDayEvent {
            year,
            greetings: Vec::new(),
        }
    }

    pub fn add_greeting(&mut self, greeting: Greeting) {
        self.greetings.push(greeting);
    }

    pub fn run(&self) {
        println!("\n🇺🇦 ========================================");
        println!("   Ukraine Independence Day Celebration");
        println!("        August 24, {}", self.year);
        println!("========================================🇺🇦\n");

        println!("Greetings from around the world:\n");
        for greeting in &self.greetings {
            greeting.display();
        }

        println!("\n🇺🇦 Glory to Ukraine! 🇺🇦\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let event = IndependenceDayEvent::new(2024);
        assert_eq!(event.year, 2024);
    }

    #[test]
    fn test_add_greeting() {
        let mut event = IndependenceDayEvent::new(2024);
        let greeting = Greeting::new(
            "З Днем Незалежності України!".to_string(),
            Language::Ukrainian,
        );
        event.add_greeting(greeting);
        assert_eq!(event.greetings.len(), 1);
    }
}

pub fn main() {
    let mut event = IndependenceDayEvent::new(2024);

    event.add_greeting(Greeting::new(
        "З Днем Незалежності України! Слава Україні!".to_string(),
        Language::Ukrainian,
    ));

    event.add_greeting(Greeting::new(
        "Happy Independence Day to all Ukrainians!".to_string(),
        Language::English,
    ));

    event.add_greeting(Greeting::new(
        "Joyeuses félicitations à l'indépendance de l'Ukraine!".to_string(),
        Language::French,
    ));

    event.run();
}
