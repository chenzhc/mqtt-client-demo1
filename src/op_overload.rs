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


struct GroceryItem {
    name: String,
    price: f32,
}

struct GroceryBill {
    items: Vec<GroceryItem>,
    tax_rate: f32,

}

impl GroceryBill {

    fn calculate_total(&self) -> f32 {
        let items_total = self.items
            .iter()
            .fold(0f32, |a, item|{
                return a + item.price;
            });
        let tax_value = items_total * self.tax_rate;
        return items_total + tax_value;
    }
}


impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;
    fn add(self, rhs: GroceryItem) -> Self::Output {
        let mut bill = self;
        bill.items.push(rhs);
        
        return bill;
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_new_bill_test() {
        crate::init();
        let mut new_bill = GroceryBill {
            items: Vec::new(),
            tax_rate: 0.027,
        };

        let carrots = GroceryItem {
            name: "Bag of Carrots 1 pound".to_string(),
            price: 10.5,
        };

        let cheese = GroceryItem {
            name: "Cottage Cheese 12oz.".to_string(),
            price: 3.4,
        };

        new_bill = new_bill + carrots + cheese;

        let total = new_bill.calculate_total();
        info!("The total of your grocery bill is: {total}");

    }

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