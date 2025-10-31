use std::fmt::Display;

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
}
