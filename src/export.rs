use firebase_rs::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ep {
    name: String,
    player: String,
    type_video: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Content {
    eps: Vec<Ep>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Anime {
    anime: String,
    background: String,
    dados: Vec<Content>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Struture {
    animes: Vec<Anime>
}

pub fn export()
{
    let firebase = Firebase::new("https://lowstreamcast-default-rtdb.firebaseio.com").unwrap();
    let animes = firebase.at("/").unwrap();
    let res = animes.get_generic::<Struture>().unwrap();
    // let res = animes.get().unwrap();
    println!("{:#?}", res);
    println!("Hello, world!");
}
