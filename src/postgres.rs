// src/postgres.rs

use serde::Deserialize;
use std::fs;
use tokio_postgres::{Client, NoTls, Error};

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Deserialize)]
pub struct QueryConfig {
    pub sql: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub query: QueryConfig,
}

pub async fn connect_and_query() -> Result<(), Error> {
    // Read the database.toml file
    let config_contents = fs::read_to_string("database.toml").expect("Unable to read file");

    // Parse the TOML configuration
    let config: Config = toml::from_str(&config_contents).expect("Unable to parse TOML");

    // Construct the connection string
    let connection_string = format!(
        "host={} user={} password={} dbname={} port={}",
        config.database.host,
        config.database.username,
        config.database.password,
        config.database.db_name,
        config.database.port
    );

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // Spawn a new task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Execute the query
    let rows = client.query(&config.query.sql, &[]).await?;

    // Process the results dynamically
    for row in rows {
        // Get the column names from the row
        let columns = row.columns();
        
        // Print each column and its corresponding value
        for (i, column) in columns.iter().enumerate() {
            let column_name = column.name();
            // Get the value as an Option
            let value: Option<String> = row.try_get(i).unwrap_or_else(|_| None);
            
            // Match on the Option to format output
            match value {
                Some(v) => println!("{}: {}", column_name, v), // Print the value
                None => println!("{}: NULL", column_name), // Indicate NULL value
            }
        }
        println!("---"); // Separator for each row
    }

    Ok(())
}

