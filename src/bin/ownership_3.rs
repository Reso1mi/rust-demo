pub fn main() {
    {
        struct Str {
            raw_str: Option<String>,
        }

        let mut raw = Str {
            raw_str: Some("123".to_string()),
        };

        let mut s = &raw;

        let v = match &s.raw_str {
            Some(v) => v,
            None => &"1".to_string(),
        };
    }

    {
        struct Int {
            raw_int: Option<u32>,
        }

        let mut raw = Int { raw_int: Some(123) };

        let mut s = &mut raw;

        let v = match s.raw_int {
            Some(v) => v,
            None => 1,
        };
    }
}
