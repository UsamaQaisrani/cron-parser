pub struct FieldDef {
    pub name: &'static str,
    pub min: u32,
    pub max: u32,
}

pub const FIELDS: [FieldDef; 5] = [
    FieldDef {
        name: "minute",
        min: 0,
        max: 59,
    },
    FieldDef {
        name: "hour",
        min: 0,
        max: 23,
    },
    FieldDef {
        name: "day of month",
        min: 1,
        max: 31,
    },
    FieldDef {
        name: "month",
        min: 1,
        max: 12,
    },
    FieldDef {
        name: "day of week",
        min: 0,
        max: 6,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_for_minute_is_correct() {
        assert_eq!(FIELDS[0].min, 0);
        assert_eq!(FIELDS[0].max, 59);
    }

    #[test]
    fn test_min_max_for_hour_is_correct() {
        assert_eq!(FIELDS[1].min, 0);
        assert_eq!(FIELDS[1].max, 23);
    }

    #[test]
    fn test_min_max_for_day_of_month_is_correct() {
        assert_eq!(FIELDS[2].min, 1);
        assert_eq!(FIELDS[2].max, 31);
    }

    #[test]
    fn test_min_max_for_month_is_correct() {
        assert_eq!(FIELDS[3].min, 1);
        assert_eq!(FIELDS[3].max, 12);
    }

    #[test]
    fn test_min_max_for_day_of_week_is_correct() {
        assert_eq!(FIELDS[4].min, 0);
        assert_eq!(FIELDS[4].max, 6);
    }

    #[test]
    fn test_correct_fields_order() {
        assert_eq!(FIELDS[0].name, "minute");
        assert_eq!(FIELDS[1].name, "hour");
        assert_eq!(FIELDS[2].name, "day of month");
        assert_eq!(FIELDS[3].name, "month");
        assert_eq!(FIELDS[4].name, "day of week");
    }
}
