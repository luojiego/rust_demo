use clap::{AppSettings, Clap};
use anyhow::{anyhow, Result};
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use mime::Mime;
use colored::*;

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Clap, Debug)]
#[clap(version = "1.0", author="Luo Jie<13369189710@163.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key, 迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们实现了 KvPair 实现了 FromStr，这里直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_header(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).map(|v|v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v.to_string().contains("application/json") => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_header(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

#[tokio::main]
async fn main() ->Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);
    // let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERD-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        Subcommand::Get(ref args) => get(client, args).await?,
        Subcommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
