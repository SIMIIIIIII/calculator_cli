use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

const FILENAME: &str = "history.txt";
const MAX_HISTORY_ENTRIES: usize = 100;

pub fn open_history_file() -> std::io::Result<fs::File> {
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(FILENAME)?;
    Ok(file)
}

pub fn save_history(entry: &str, result: f64, file: &mut fs::File) -> std::io::Result<()> {
    let new_entry = format!("{} = {}", entry.trim(), result);

    file.seek(SeekFrom::Start(0))?;
    let mut entries: Vec<String> = {
        let reader = BufReader::new(&mut *file);
        reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()?
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect()
    };

    entries.push(new_entry);

    if entries.len() > MAX_HISTORY_ENTRIES {
        let keep_from = entries.len() - MAX_HISTORY_ENTRIES;
        entries = entries.split_off(keep_from);
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    for line in entries {
        writeln!(file, "{line}")?;
    }

    file.flush()?;
    Ok(())
}

pub fn print_history(file: &mut fs::File) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(0))?;
    
    let lecteur = BufReader::new(file);
    
    for ligne in lecteur.lines() {
        let texte_ligne = ligne?; 
        println!("Ancien calcul : {}", texte_ligne);
    }

    Ok(())
}
