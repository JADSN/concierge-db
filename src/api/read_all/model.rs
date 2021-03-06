use rusqlite::{Result, NO_PARAMS};

use crate::api::Coluna;
use crate::api::ColunaData;
use crate::database::DbConnection;


pub fn read_all(conn: DbConnection, table: String) -> Result<Vec<Vec<Coluna>>> {
    let query = format!("SELECT * FROM {};", table);
    println!("{}", &query);
    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map(NO_PARAMS, |row| {
        let num_cols = row.column_count();
        let mut colunas: Vec<Coluna> = Vec::new();
        colunas.reserve(num_cols);
        let column = &row.columns();
        // TODO: for i in 0..num_cols {
        // TODO: column.iter().enumerate().take(num_cols)
        for idx in 0..num_cols {
            match column[idx].decl_type().unwrap() {
                "INTEGER" => {
                    let result: Result<i32> = row.get(idx);
                    match result {
                        Ok(integer) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Integer(integer),
                            });
                        }
                        Err(_) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Null(()),
                            });
                        }
                    }
                }
                "TEXT" => {
                    let result: Result<String> = row.get(idx);
                    match result {
                        Ok(text) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Text(text),
                            });
                        }
                        Err(_) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Null(()),
                            });
                        }
                    }
                }
                "BOOLEAN" => {
                    let result: Result<bool> = row.get(idx);
                    match result {
                        Ok(boolean) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Boolean(boolean),
                            });
                        }
                        Err(_) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Null(()),
                            });
                        }
                    }
                }
                "DATETIME" => {
                    let result: Result<i32> = row.get(idx);
                    match result {
                        Ok(epoch) => {
                            let datetime: i32 = if epoch > 0 { epoch } else { 0 };
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Datetime(datetime),
                            });
                        }
                        Err(_) => {
                            colunas.push(Coluna {
                                column_name: column[idx].name().to_string(),
                                data: ColunaData::Null(()),
                            });
                        }
                    }
                }
                _ => {
                    // Outros tipos: REAL, NULL, BLOB
                    dbg!("TypeName unknown");
                    dbg!(column[idx].decl_type());
                    let data: String = row.get(idx).unwrap();
                    dbg!(&data);
                    colunas.push(Coluna {
                        column_name: column[idx].name().to_string(),
                        data: ColunaData::Unknown(data),
                    });
                }
            }
        }
        Ok(colunas)
    })?;
    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
