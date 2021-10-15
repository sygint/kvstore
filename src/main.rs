use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("The key is {} and the value is {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents);

    let database = Database::new().expect("Creating db failed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // read kv.db file
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // let chunks = line.splitn(2, '\t');
            // let key = chunks.next().expect("No key!");
            // let value = chunks.next().expect("No value!");
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        // // parse string
        // populate our map
        Ok(Database { map: map })
    }
}
