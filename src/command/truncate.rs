use crate::command::sqlcommands::SqlCommand;
use sqlparser::ast::Truncate;

pub fn parse(truncate: Truncate) -> SqlCommand {
    let tables = truncate
        .table_names
        .iter()
        .map(|table_target| table_target.name.to_string())
        .collect();

    SqlCommand::TRUNCATE {
        command: String::from("TRUNCATE"),
        tables,
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::command::sqlcommands::SqlCommand;
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    fn parse_truncate_sql(sql: &str) -> SqlCommand {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        match ast.into_iter().next().unwrap() {
            Statement::Truncate(truncate) => parse(truncate),
            other => panic!("expected TRUNCATE statement, got {:?}", other),
        }
    }

    #[test]
    fn parses_single_table() {
        let result = parse_truncate_sql("TRUNCATE TABLE employee;");

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![String::from("employee")],
            }
        );
    }

    #[test]
    fn parses_single_table_without_trailing_semicolon() {
        let result = parse_truncate_sql("TRUNCATE TABLE employee");

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![String::from("employee")],
            }
        );
    }

    #[test]
    fn parses_multiple_tables() {
        let result = parse_truncate_sql("TRUNCATE TABLE employee, department, salary;");

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![
                    String::from("employee"),
                    String::from("department"),
                    String::from("salary"),
                ],
            }
        );
    }

    #[test]
    fn preserves_schema_qualified_names() {
        let result = parse_truncate_sql("TRUNCATE TABLE public.employee, hr.department;");

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![
                    String::from("public.employee"),
                    String::from("hr.department"),
                ],
            }
        );
    }

    #[test]
    fn preserves_quoted_identifiers() {
        let result = parse_truncate_sql(r#"TRUNCATE TABLE "employee", "order";"#);

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![
                    String::from("\"employee\""),
                    String::from("\"order\""),
                ],
            }
        );
    }

    #[test]
    fn preserves_quoted_schema_qualified_names() {
        let result = parse_truncate_sql(r#"TRUNCATE TABLE "public"."employee", "hr"."payroll";"#);

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![
                    String::from("\"public\".\"employee\""),
                    String::from("\"hr\".\"payroll\""),
                ],
            }
        );
    }

    #[test]
    fn command_string_is_always_truncate() {
        let result = parse_truncate_sql("TRUNCATE TABLE employee;");

        match result {
            SqlCommand::TRUNCATE { command, tables } => {
                assert_eq!(command, "TRUNCATE");
                assert_eq!(tables, vec![String::from("employee")]);
            }
            other => panic!("expected SqlCommand::TRUNCATE, got {:?}", other),
        }
    }

    #[test]
    fn preserves_table_order() {
        let result = parse_truncate_sql("TRUNCATE TABLE z_table, a_table, middle_table;");

        match result {
            SqlCommand::TRUNCATE { tables, .. } => {
                assert_eq!(
                    tables,
                    vec![
                        String::from("z_table"),
                        String::from("a_table"),
                        String::from("middle_table"),
                    ]
                );
            }
            other => panic!("expected SqlCommand::TRUNCATE, got {:?}", other),
        }
    }

    #[test]
    fn returns_empty_vector_for_empty_table_list_if_constructed_directly() {
        let dialect = GenericDialect {};
        let ast = Parser::parse_sql(&dialect, "TRUNCATE TABLE employee;").unwrap();

        let truncate = match ast.into_iter().next().unwrap() {
            Statement::Truncate(truncate) => truncate,
            other => panic!("expected TRUNCATE statement, got {:?}", other),
        };

        let mut empty_truncate = truncate;
        empty_truncate.table_names.clear();

        let result = parse(empty_truncate);

        assert_eq!(
            result,
            SqlCommand::TRUNCATE {
                command: String::from("TRUNCATE"),
                tables: vec![],
            }
        );
    }
}
