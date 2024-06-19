fn main() {
    #[allow(deprecated)]
    let home = std::env::home_dir().unwrap();
    let work = std::env::current_dir().unwrap();
    
    if let Some(keys) = std::env::var("ONEENV_KEYS").ok() {
        keys.split(',')
            .for_each(|key| {
                println!("unset {key}");
            })
    }

    let mask = format!("{}:", work.to_str().unwrap());
    let data = std::fs::read_to_string(&home.join(".oneenv")).unwrap();
    let lines = data.split('\n').collect::<Vec<_>>();
    if let Some(chunk) = lines
        .split(|s| s.is_empty())
        .filter(|chunk| !chunk.is_empty())
        .find(|chunk| chunk[0] == &mask) {
            let mut keys: Vec<String> = Vec::with_capacity(chunk.len());
            chunk
                .iter()
                .skip(1)
                .filter_map(|line| line.split_once('='))
                .for_each(|(key, value)| {
                    keys.push(key.to_owned());
                    let value = value.trim_end();
                    println!("export {key}='{value}'")
                });
            println!("ONEENV_KEYS={}", keys.join(","))
        }
}
