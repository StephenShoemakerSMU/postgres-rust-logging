use postgres::{Client, NoTls};


fn main(){
    let mut client = Client::connect("host=localhost user=user password=example", NoTls).unwrap();
    for row in &client.query("SELECT id FROM logs", &[]).unwrap(){
        let id: i16 = row.get(0);
        println!("{}", id);
    }
}