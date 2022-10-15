//Network mod
mod network;
mod files;

#[tokio::main]
async fn main() {
    let end_point = "https://jsonplaceholder.typicode.com/posts".to_string();
    let mut response: Vec<String> = network::request::send_request(&end_point).await.expect("Endpoint cant be called");

    //Store response 
    
    files::file::strip_dot(response);
   // let write = files::file::create_file(&response, &end_point);
   // match write {
   //     Ok(_) => println!("Write successful."),
   //     Err(err) => println!("{:?}", err),
   // }


}
