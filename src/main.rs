mod postgres; // Declare the module

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    // Call the function from the postgres module
    postgres::connect_and_query().await?;
    Ok(())
}
