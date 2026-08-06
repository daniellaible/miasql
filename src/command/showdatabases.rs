use anyhow::{Error};
use crate::command::resultset::ResultSet;
use crate::command::sqlcommands::SqlCommand;
use crate::database::bptree::Node;
use crate::database::table::{Row};
use crate::server::dbmem::DbMem;

pub fn parse() -> SqlCommand {
    SqlCommand::ShowDatabases {
        command: String::from("SHOW DATABASES"),
    }
}

pub fn show_databases() -> anyhow::Result<ResultSet, Error> {
    let mut result:ResultSet = ResultSet::create();
    if let Some(table_arc) = DbMem::find_table_in_mem("system", "database") {
        let table_guard = table_arc.lock().unwrap();
        let tree = &table_guard.tree;
        let header = &table_guard.column_names;
        result.header = header.clone();

        let mut cur = Some(tree.leftmost_leaf(tree.root.clone()));

        while let Some(node_arc) = cur {
            let (rows_to_send, next_leaf) = {
                let node_guard = node_arc.lock().unwrap();
                let Node::Leaf(leaf) = &*node_guard else {
                    unreachable!("leftmost_leaf/next chain must be leaves");
                };
                (leaf.values.clone(), leaf.next.clone())
            };

            for raw_row in rows_to_send {
                let row: Row = Row {
                    data: raw_row
                };
                result.rows.push(row);
            }
            cur = next_leaf;
        }
    }
    anyhow::Ok(result)
}


