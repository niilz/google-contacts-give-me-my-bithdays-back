use std::fmt::Display;

use crate::api::calendar::{Date, InsertEvent};
use crate::api::person::Person;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Birthday {
    name: String,
    day: u32,
    month: u32,
    year: Option<u32>,
}

impl PartialOrd for Birthday {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Birthday {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.month
            .cmp(&other.month)
            .then(self.day.cmp(&other.day))
            .then(self.year.cmp(&other.year))
            .then(self.name.cmp(&other.name))
    }
}

impl TryFrom<Person> for Birthday {
    type Error = &'static str;
    fn try_from(person: Person) -> Result<Self, Self::Error> {
        match person.birthdays {
            Some(birthdays) if birthdays.len() > 0 => {
                let birthday = &birthdays[0];
                Ok(Birthday {
                    name: person.names[0].display_name.to_string(),
                    day: birthday.date.day,
                    month: birthday.date.month,
                    year: birthday.date.year,
                })
            }
            _ => Err("person had no birthdays"),
        }
    }
}

impl From<&Birthday> for InsertEvent {
    fn from(birthday: &Birthday) -> Self {
        let year = match birthday.year {
            Some(year) => year,
            None => 2020,
        };
        let start = format!("{year}-{:02}-{:02}", birthday.month, birthday.day);
        let end = format!("{year}-{:02}-{:02}", birthday.month, birthday.day + 1);
        Self {
            start: Date { date: start },
            end: Date { date: end },
            summary: format!("🎁{}", birthday.name),
            event_type: "default".to_string(),
            transparency: "transparent".to_string(),
            visibility: "private".to_string(),
            recurrence: vec!["RRULE:FREQ=YEARLY".to_string()],
        }
    }
}

impl Display for Birthday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.year {
            Some(year) => write!(f, "{}: {}.{}.{}", self.name, self.day, self.month, year),
            None => write!(f, "{}, {}.{}", self.name, self.day, self.month),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Birthday;
    use crate::api::calendar::InsertEvent;

    #[test]
    fn birthday_with_earlier_month_comes_first() {
        let june = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 6,
            year: Some(1990),
        };
        let july = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 7,
            year: Some(1990),
        };

        let mut birthdays = vec![july.clone(), june.clone()];
        birthdays.sort();

        assert_eq!(birthdays, vec![june, july])
    }

    #[test]
    fn birthday_month_wins_over_year_comes_first() {
        let june = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 6,
            year: Some(2000),
        };
        let july = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 7,
            year: Some(1990),
        };

        let mut birthdays = vec![july.clone(), june.clone()];
        birthdays.sort();

        assert_eq!(birthdays, vec![june, july])
    }

    #[test]
    fn birthday_day_wins_over_year_on_equal_months() {
        let june = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 6,
            year: Some(2000),
        };
        let june_later = Birthday {
            name: "dummy".to_string(),
            day: 15,
            month: 6,
            year: Some(1990),
        };

        let mut birthdays = vec![june_later.clone(), june.clone()];
        birthdays.sort();

        assert_eq!(birthdays, vec![june, june_later])
    }

    #[test]
    fn birthday_to_event_with_year() {
        // given
        let june = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 6,
            year: Some(2000),
        };

        let expected_event = InsertEvent {
            summary: format!("🎁dummy"),
            start: "2000-06-01".into(),
            end: "2000-06-02".into(),
            event_type: "default".to_string(),
            transparency: "transparent".to_string(),
            visibility: "private".to_string(),
            recurrence: vec!["RRULE:FREQ=YEARLY".to_string()],
        };

        // when
        let event: InsertEvent = (&june).into();

        assert_eq!(event, expected_event);
    }

    #[test]
    fn birthday_to_event_no_year() {
        // given
        let june = Birthday {
            name: "dummy".to_string(),
            day: 1,
            month: 6,
            year: None,
        };

        let expected_event = InsertEvent {
            summary: format!("🎁dummy"),
            start: "2020-06-01".into(),
            end: "2020-06-02".into(),
            event_type: "default".to_string(),
            transparency: "transparent".to_string(),
            visibility: "private".to_string(),
            recurrence: vec!["RRULE:FREQ=YEARLY".to_string()],
        };

        // when
        let event: InsertEvent = (&june).into();

        assert_eq!(event, expected_event);
    }
}
