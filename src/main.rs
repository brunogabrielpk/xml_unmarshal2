use serde::{Deserialize, Serialize};
// use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_xml_rs::from_str;

// use std::env;
use std::fs::File;
use std::io::Read;

// there is  file named wkflow.xml to be read and stored in the rust structs below

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    #[serde(rename = "name")]
    name: String,
    // #[serde(rename = "$value")]
    // value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Arg {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Function {
    #[serde(rename = "arg")]
    value: Vec<Arg>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UnconditionalResult {
    #[serde(rename = "post-functions")]
    post_functions: Vec<Function>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    #[serde(rename = "unconditional-result")]
    unconditional_result: UnconditionalResult,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Results {
//     #[serde(rename = "$value")]
//     results: Vec<Result>,
// }

#[derive(Debug, Deserialize, Serialize)]
struct Validator {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(rename = "$value")]
    value: Vec<Arg>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Validators {
//     #[serde(rename = "validator")]
//     validator: Vec<Validator>,
// }

#[derive(Debug, Serialize, Deserialize)]
enum ActionEvent {
    Meta(Vec<Meta>),
    Validators(Vec<Validator>),
    Results(Vec<Result>),
}

#[derive(Debug, Deserialize, Serialize)]
struct Action {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "$value")]
    events: Vec<ActionEvent>,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Step {
//     #[serde(rename = "meta")]
//     meta: Vec<Meta>,
//     #[serde(rename = "action")]
//     actions: Vec<Action>,
// }

#[derive(Debug, Deserialize, Serialize)]
struct Workflow {
    // #[serde(rename = "step")]
    // steps: Vec<Step>,
    //#[serde(rename = "global-actions")]
    // global_actions: Vec<Action>,
    // #[serde(rename = "common-actions")]
    // common_actions: Vec<Action>,
    #[serde(rename = "initial-actions")]
    initial_actions: Vec<Action>,
    #[serde(rename = "meta")]
    meta: Vec<Meta>,
}
fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    println!("{}", std::env::current_dir().unwrap().display());
    let mut file = File::open("./src/wkflow.xml").expect("Failed to open file AAAAAAAA");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let workflow: Workflow = from_str(&contents).expect("Failed to deserialize the XML file");
    println!("{:#?}", workflow)
}
