use crate::database;
use log::info;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::database::database::Database;

pub fn read_system_table() -> Database {
    if let Ok(lines) = read_lines("C:\\Miasql\\system\\system.mos") {
        for line in lines.map_while(Result::ok) {
            let mut splits = line.trim().split(";");

            let _: i32 = match splits.next(){
                Some(id) => id.parse::<i32>().unwrap(),
                None => return Database::default()
            };

            let dbname: &str = match splits.next(){
                Some(dbname) => dbname,
                None => return Database::default()
            };

            let tablename: &str = match splits.next(){
                Some(tablename) => tablename,
                None => return Database::default()
            };

            let path: &str = match splits.next(){
                Some(path) => path,
                None => return Database::default()
            };

            let mut database = Database::default();
            database.set_db_name(dbname.to_string());
            database.add_table( tablename.to_string(), path.to_string() );

            return database;
        }

    } else {
        //TODO here we need to create a new database for system.tables and system.users
    }
    Database::default()
}

pub fn read_users() {
    todo!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {}