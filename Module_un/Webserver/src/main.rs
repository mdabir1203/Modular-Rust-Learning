extern crate iron;
#[macro_use] extern crate mime; // directives to use macro
extern crate router;

use router::Router;
use iron::prelude::*; // makes all module available
use iron::status;

fn main() {
    println!("Running on http://localhost:5050..");
    Iron::new(get_form).http("localhost:5050").unwrap();
}

// takes a mutable request
fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>Web Calculator"</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute Server Points</button
        </form
        "#);
        println!("what is going on");
        Ok(response) // specify the return value
        
}