#![allow(dead_code)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Base<T1, T2, T3, T4, T5, T6> {
    item1: T1,
    item2: T2,
    item3: T3,
    item4: T4,
    item5: T5,
    item6: T6,
}

pub type Single<T> = Base<T, T, T, T, T, T>;

pub type Body = Single<Option<bool>>;

pub fn parse_body(body: &str) -> Option<Body> {
    serde_json::from_str(body).ok()
}
