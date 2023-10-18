use postgres::{Client, NoTls};

pub fn establish_connection() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
}
