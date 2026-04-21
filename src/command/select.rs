use std::mem::take;
use crate::command::whereclause::WhereClause;

pub struct Select {
    table_name: String,
    columns: Vec<String>,
    where_clause: WhereClause,
}

pub fn parse_stmt(stmt: String) -> Select {

    let select: Select = Select::default();
    select
}

impl Select {
    pub fn default() -> Self {
        Select{
            table_name: String::default(),
            columns: vec!(),
            where_clause: WhereClause::default()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::command::select::{parse_stmt, Select};

    #[test]
    fn simple_select_with_where_clause() {
        let select = "SELECT name, country FROM population WHERE id=1";
        parse_stmt(String::from(select));
    }
}