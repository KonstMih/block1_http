extern crate iron;
extern crate router;
extern crate urlencoded;


//#[macro_use] 
extern crate mime;

use iron::prelude::*;
use iron::status;

use router::Router;

//use std::str::FromStr;
//use urlencoded::UrlEncodedBody;
use std::fs;


fn main() {
    let mut router = Router::new();
    
    router.get("/", get_form, "root");
    //router.post("/gcd", post_gcd, "gcd");    
    
	println!("Serving on http://localhost:3000...");
	Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {

	    let content = fs::read_to_string("data.json").expect("Файл не прочитался");
    	//let content = fs::read_to_string("/home/asutp/data.json").expect("Файл не прочитался");
    	let rows = rm_head(content);
    	
		let mut response = Response::new();
		response.set_mut(status::Ok);
		response.set_mut(rows);
		
		Ok(response)
}


fn rm_head(mut rows_str: String) -> String {
    let num:usize;
    let data: String;
    if rows_str.len() > 68655 {
        num = rows_str.len() - 68655;
        data = rows_str.split_off(num);
    } else {
        data = rows_str;
    }
    let rows = data.split_once("{").unwrap();
    let mut tail = rows.1.to_string();
    tail.insert_str(0, "{");
    tail.trim().to_string();
    tail
}