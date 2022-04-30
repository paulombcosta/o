use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cmd {
    pub name: String,
    pub value: Option<String>,
    pub children: Option<Vec<Cmd>>,
}
