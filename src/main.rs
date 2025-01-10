mod postgres; // Declare the module
mod config; // Declare the config module

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let matches = config::get_matches();

    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    let config_path = match matches.value_of("config") {
        Some(path) => path,
        None => {
            eprintln!("Please specify a database file using the -c or --config options.");
            std::process::exit(1);
        }
    };
    println!("Using config file: {}", config_path);

    // Call the function from the postgres module with the config path
    postgres::connect_and_query(config_path).await?;
    Ok(())
}