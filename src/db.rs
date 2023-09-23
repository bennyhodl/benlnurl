use rusqlite::Connection;

pub fn connect_to_database() -> Connection {
    let db = match Connection::open("benlnurl.db") {
        Ok(database) => database,
        Err(e) => {
            println!("Error opening the database: {}", e);
            panic!();
        }
    };

    let crete_table_query = "
        create table if not exists benlnurl (
            username text not null unique,
            minSpendable integer not null,
            maxSpendable integer not null,
            address text not null,
            macaroon text not null,
            cert text not null
        )
    ";

    match db.execute(crete_table_query, ()) {
        Ok(..) => println!("Using benlnurl database"),
        Err(e) => {
            println!("Did not create table: {:?}", e);
            panic!()
        }
    };

    db
}
