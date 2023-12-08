#![allow(unused)]
use reqwest::Client;
use sqlx::postgres::PgPool;
use tracing::error;

struct CustomService {
    ctx: Client,
    db: PgPool
}

// Set up our user agent
const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";


fn main(){

}
