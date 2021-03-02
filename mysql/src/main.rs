use mysql::*;
use mysql::prelude::*;
use mysql::MySqlError;

#[derive(Debug, PartialEq, Eq)]
struct AccountData {
    id: i32,
    account: Option<String>,
    password: Option<String>,
}

use std::env;

// fn main() -> Result<(), MySqlError> {
fn main() {
    // TODO 密码从环境变量中读取
    let password = env::var("DEFAULT_MYSQL_PASSWORD").expect("get default mysql password err");

    let url = format!("mysql://root:{}@192.168.196.16:3306/game_db", password);
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) =>  {
            // let s =  format!("x parse err: {:?}", e);
            // return Err(Box::leak(s.into_boxed_str()));
            panic!("connect mysql err: {:?}", e);
        }
    };

    let mut conn = pool.get_conn().expect("get conn err");

    let selected_accounts = conn.query_map(
        "SELECT user_id, account, password from account_data LIMIT 10",
        |(id, account, password)|{
            AccountData {
                id,
                account,
                password,
            }
        }
    ).expect("select error");

    println!("result: {:#?}", selected_accounts);
}
