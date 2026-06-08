use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_system_table() {
    if let Ok(lines) = read_lines("C:\\Miasql\\system\\system.mos") {
        for line in lines.map_while(Result::ok) {
            let mut splits = line.trim().split(";");
            let mut system_columns = Vec::<String>::new();

            for column in splits {
                system_columns.push(column.to_string());
            }

            let dbname = &system_columns[0];
            let tablename = &system_columns[1];
            let path = &system_columns[2];

        }
    } else {}
}

pub fn read_users(){
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
mod tests {


}