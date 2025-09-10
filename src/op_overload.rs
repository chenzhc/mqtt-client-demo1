#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Add;



#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
pub struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::NaiveDate,
}

impl Add for Person {
    type Output = Marriage;

    fn add(self, rhs: Self) -> Self::Output {
        return Marriage {
            husband: self,
            wife: rhs,
            location: "Arizona".to_string(),
            date: chrono::offset::Local::now().date_naive(),
        }
    }
}


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[tokio::test]
    async fn it_test_db01() {
        crate::init();
        info!("This is an info message from the test.");

        let person1 = Person {
            first_name: "Trevor".to_string(),
            last_name: "Sullivan".to_string(),
        };

        let person2 = Person {
            first_name: "Nacy".to_string(),
            last_name: "Jones".to_string(),
        };

        let marriage = person1 + person2;
        info!("{} got married to {} on {} date", 
            marriage.husband.first_name,
            marriage.wife.first_name,
            marriage.date    
        );
        
    }
}