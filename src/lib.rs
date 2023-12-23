#![no_main]

use extism_pdk::*;
use serde::{Serialize, Deserialize};


// return a hashmap of array of strings
fn headers_map() ->     std::collections::HashMap<String, Vec<String>> {
    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
    headers
}

#[derive(Serialize)]
struct ResponseData {
    pub body: String,
    pub header: std::collections::HashMap<String, Vec<String>>,
    pub code: i32,
}

#[derive(Serialize, Deserialize)]
struct RequestData {
    pub body: String,
    pub header: std::collections::HashMap<String, Vec<String>>,
    pub method: String,
    pub uri: String,
}

// create a function name ant that take a String as parameter and return another String
// #[plugin_fn("handle")]
pub fn get_ant(input: String) -> String {
format!(r#"
/\/\
  \_\  _..._
  (" )(_..._)
   ^^  // \\
{}
"#, input).to_string()
}


#[plugin_fn]
pub fn handle(input: String) -> FnResult<Json<ResponseData>> {

    let req: RequestData = serde_json::from_str(&input).unwrap();

    let message: String = get_ant(req.body);

    let resp = ResponseData { 
        body: message , 
        code: 200, 
        header: headers_map()
    };
    
    Ok(Json(resp))
}
