#[derive(Debug, Clone)]
pub struct Event {
    pub name: &'static str,
    pub month: u8,
    pub day: u8,
    pub message: &'static str,
}

impl Event {
    pub fn is_today(&self, month: u8, day: u8) -> bool {
        self.month == month && self.day == day
    }

    pub fn formatted_message(&self) -> String {
        format!("{} — {} ({}-{})", self.name, self.message, self.month, self.day)
    }
}

pub fn st_georges_day() -> Event {
    Event {
        name: "St. George's Day",
        month: 4,
        day: 23,
        message: "Wishing all in England a happy St. George's Day!",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_georges_day_date() {
        let ev = st_georges_day();
        assert_eq!(ev.month, 4);
        assert_eq!(ev.day, 23);
    }

    #[test]
    fn test_message_format() {
        let ev = st_georges_day();
        let msg = ev.formatted_message();
        assert!(msg.contains("St. George"));
        assert!(msg.contains("happy St. George's Day"));
    }
}
