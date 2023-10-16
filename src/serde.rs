
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use serde::{Deserialize, Serialize};

    static DATA: LazyLock<PathBuf, fn() -> PathBuf> =
        LazyLock::new(|| std::env::current_dir().unwrap().join("data"));


    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct PersonMini {
        name: String,
        age: u8,
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct PersonWrong {
        name: String,
        age: u8,
        emails: Vec<String>,
    }

    #[test]
    fn read_json() -> io::Result<()> {
        let p = DATA.join("file.json");
        let f = File::open(p)?;

        let p: Person = serde_json::from_reader(f)?;
        let john = Person {
            name: "John Doe".to_string(),
            age: 43,
            phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()]
        };
        assert_eq!(p, john);
        Ok(())
    }

    #[test]
    fn read_json_wrong() -> io::Result<()> {
        let p = DATA.join("file.json");
        let f = File::open(p)?;

        let result = serde_json::from_reader::<_, PersonWrong>(f);
        assert_eq!(result.map_err(|e| e.is_data()), Err(true));
        Ok(())
    }

    #[test]
    fn read_json_mini() -> io::Result<()> {
        let p = DATA.join("file.json");
        let f = File::open(p)?;

        let p: PersonMini = serde_json::from_reader(f)?;
        assert_eq!(p, PersonMini { name: "John Doe".to_string(), age: 43 });
        Ok(())
    }

    fn split_phones(mut person: Person) -> Person {
        if let Some(phone) = person.phones.pop() {
            person.phones.extend(phone.split(',').map(str::to_owned));
        }
        person
    }

    #[test]
    fn read_csv() -> io::Result<()> {
        let p = DATA.join("file.csv");
        let f = File::open(p)?;

        let mut rdr = csv::Reader::from_reader(f);
        let v = rdr.deserialize()
            .map(|result| result.map(split_phones))
            .collect::<Result<Vec<Person>, _>>()?;

        let john = Person {
            name: "John Doe".to_string(),
            age: 43,
            phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()]
        };
        assert_eq!(v, [john]);
        Ok(())
    }

    #[test]
    fn read_csv_mini() -> io::Result<()> {
        let p = DATA.join("file.csv");
        let f = File::open(p)?;

        let mut rdr = csv::Reader::from_reader(f);
        let v = rdr.deserialize()
            .collect::<Result<Vec<PersonMini>, _>>()?;

        let john = PersonMini {
            name: "John Doe".to_string(),
            age: 43,
        };
        assert_eq!(v, [john]);
        Ok(())
    }
}
