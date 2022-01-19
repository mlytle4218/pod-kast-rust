mod api {
    pub mod api;
}

mod menu {
    pub mod menu;
}

mod data {
    // pub mod data;
    pub mod category;
    pub mod podcast;
    pub mod episode;
    pub mod context;
}

fn main()  {
    let result = api::api::search("News").unwrap();
    // match api::api::search("News"){
    //     Ok(result) => println!("{:?}", result),
    //     Err(err) => println!("{}", err)
    // }
    println!("{:?}", result);
}