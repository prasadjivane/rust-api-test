mod api;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <method> <url> [body]", args[0]);
        return;
    }

    let method = &args[1];
    let url = &args[2];

    let body: HashMap<&str, &str> = if args.len() > 3 {
        args[3..].iter()
            .map(|s| {
                let mut parts = s.split('=');
                (parts.next().unwrap(), parts.next().unwrap_or(""))
            })
            .collect()
    } else {
        HashMap::new()
    };

    let result = match method.as_str() {
        "GET" => tokio::runtime::Runtime::new().unwrap().block_on(api::get(url)),
        "POST" => tokio::runtime::Runtime::new().unwrap().block_on(api::post(url, body)),
        "PUT" => tokio::runtime::Runtime::new().unwrap().block_on(api::put(url, body)),
        "DELETE" => tokio::runtime::Runtime::new().unwrap().block_on(api::delete(url)),
        _ => {
            println!("Invalid method!");
            return;
        }
    };

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }
}
