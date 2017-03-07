extern crate odbc;
use odbc::*;

#[test]
fn list_tables() {

    let mut env = Environment::new().unwrap();
    let mut ds = DataSource::with_dsn_and_credentials(&mut env, "PostgreSQL", "postgres", "")
        .unwrap();
    // scope is required (for now) to close statement before disconnecting
    {
        let statement = Statement::with_tables(&mut ds).unwrap();
        assert_eq!(statement.num_result_cols().unwrap(), 5);
    }
    ds.disconnect().unwrap();
}

#[test]
fn test_connection() {

    let mut environment = Environment::new().expect("Environment can be created");
    let mut conn =
        DataSource::with_dsn_and_credentials(&mut environment, "PostgreSQL", "postgres", "")
            .expect("Could not connect");

    assert!(!conn.read_only().unwrap());
    conn.disconnect().unwrap();
}

#[test]
fn list_drivers() {
    let environment = Environment::new();
    let drivers = environment.expect("Environment can be created")
        .drivers()
        .expect("Drivers can be iterated over");
    println!("{:?}", drivers);

    let expected = ["PostgreSQL ANSI", "PostgreSQL Unicode", "SQLite", "SQLite3"];
    assert!(drivers.iter().map(|d| &d.description).eq(expected.iter()));
}

#[test]
fn list_data_sources() {
    let environment = Environment::new();
    let sources = environment.expect("Environment can be created")
        .data_sources()
        .expect("Data sources can be iterated over");
    println!("{:?}", sources);

    let expected = [DataSourceInfo {
                        server_name: "PostgreSQL".to_owned(),
                        description: "PostgreSQL Unicode".to_owned(),
                    }];
    assert!(sources.iter().eq(expected.iter()));
}

#[test]
fn list_user_data_sources() {
    let environment = Environment::new();
    let sources = environment.expect("Environment can be created")
        .user_data_sources()
        .expect("Data sources can be iterated over");
    println!("{:?}", sources);

    let expected = [DataSourceInfo {
                        server_name: "PostgreSQL".to_owned(),
                        description: "PostgreSQL Unicode".to_owned(),
                    }];
    assert!(sources.iter().eq(expected.iter()));
}

#[test]
fn list_system_data_sources() {
    let environment = Environment::new();
    let sources = environment.expect("Environment can be created")
        .system_data_sources()
        .expect("Data sources can be iterated over");
    println!("{:?}", sources);

    let expected: [DataSourceInfo; 0] = [];
    assert!(sources.iter().eq(expected.iter()));
}
