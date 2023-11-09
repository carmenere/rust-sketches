use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPerson {
    name: String,
    surname: String,
    age: u8,
    address: Option<String>,
    tel: Option<String>,
}

impl From<web::Json<NewPerson>> for NewPerson {
    fn from(person: web::Json<NewPerson>) -> Self {
        println!("HERE");
        Self { 
            name: person.name.clone(),
            surname: person.surname.clone(),
            age: person.age,
            address: person.address.clone(),
            tel: person.tel.clone()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    pub id: u64,
    name: String,
    surname: String,
    age: u8,
    address: Option<String>,
    tel: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PersonUrl {
    pub id: u64,
}

impl From<web::Path<PersonUrl>> for PersonUrl {
    fn from(person: web::Path<PersonUrl>) -> Self {
        Self { 
            id: person.id,
        }
    }
}

impl Person {
    pub fn from_new (p: NewPerson) -> Self {
        Self { 
            id: 0,
            name: p.name,
            surname: p.surname,
            age: p.age,
            address: p.address,
            tel: p.tel
        }
    }
}

impl From<web::Json<Person>> for Person {
    fn from(person: web::Json<Person>) -> Self {
        Self { 
            id: person.id,
            name: person.name.clone(),
            surname: person.surname.clone(),
            age: person.age,
            address: person.address.clone(),
            tel: person.tel.clone()
        }
    }
}

impl From<web::Json<NewPerson>> for Person {
    fn from(person: web::Json<NewPerson>) -> Self {
        Self { 
            id: 0,
            name: person.name.clone(),
            surname: person.surname.clone(),
            age: person.age,
            address: person.address.clone(),
            tel: person.tel.clone()
        }
    }
}