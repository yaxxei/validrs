use validrs::rules::contains::ValidateContains;
use validrs::rules::length::ValidateLength;
use validrs::rules::range::ValidateRange;
use validrs::rules::required::ValidateRequired;
use validrs::validate::Validate as _;
use validrs_derive::Valid;

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 20,
        email: "@.".to_string(),
        url: Box::new("http://google.com".to_string()),
        allow: Some(true),
        roles: vec!["user".to_string()],
    };
    match user.validate() {
        Ok(_) => println!("Struct is valid"),
        Err(err) => println!("{err}"),
    };
}

#[derive(Debug, Valid)]
struct User {
    #[valid(len(
        min = 1,
        max = 16,
        msg = "The length must be at least {{min}} character and no more than {{max}} characters"
    ))]
    name: String,

    #[valid(rng(
        min = 18,
        max = 120,
        msg = "The age must be at least {{min}} and no more than {{max}}"
    ))]
    age: usize,

    #[valid(contains(["@", "."], msg = "The provided email is not correct"))]
    email: String,

    #[valid(contains(["http"], msg = "The provided url is not correct"))]
    url: Box<String>,

    #[valid(required)]
    allow: Option<bool>,

    #[valid(required(msg = "At least 1 role is required"))]
    roles: Vec<String>,
}
