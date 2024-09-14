struct Point{
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

enum Flavor {
    NoSugar,
    HalfSugar,
    FullSugar,
}

impl Flavor {
    fn show(&self) -> String {
        match self {
            Flavor::NoSugar => "No Sugar".to_owned(),
            Flavor::HalfSugar => "Half Sugar".to_owned(),
            Flavor::FullSugar => "Full Sugar".to_owned(),
        }
    }
}

struct Drink {
    name: String,
    price: f64,
    flavor: Flavor,
}

impl Drink {
    fn show(&self) {
        println!("{}: ${} - {:?}", self.name, self.price, self.flavor.show());
    }
}


struct Student {
    name: String,
    balance: f64,
    have_cost: f64,
}

impl Student {
    const MAX_COST: f64 = 5.0;
    fn purchase(&mut self, drink: &Drink) {
        if self.balance >= drink.price {
            if self.have_cost + drink.price > Self::MAX_COST {
                println!("{} cannot purchase {} for ${} because they have exceeded the maximum cost of ${}", self.name, drink.name, drink.price, Self::MAX_COST);
                return;
            }
            self.balance -= drink.price;
            self.have_cost += drink.price;
            println!("{} purchased a {} for ${}. New balance: ${}", self.name, drink.name, drink.price, self.balance);
        } else {
            println!("{} does not have enough money to purchase a {} for ${}", self.name, drink.name, drink.price);
        }
    }
}


fn main() {
    let point1 = Point { x: 0.0, y: 0.0 };
    let point2 = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", point1.distance(&point2));
    let drink1 = Drink {
        name: String::from("Coke"),
        price: 3.0,
        flavor: Flavor::NoSugar,
    };
    let drink2 = Drink {
        name: String::from("Pepsi"),
        price: 3.0,
        flavor: Flavor::HalfSugar,
    };
    let drink3 = Drink {
        name: String::from("Sprite"),
        price: 99.0,
        flavor: Flavor::FullSugar,
    };
    drink1.show();
    drink2.show();
    drink3.show();
    let mut student1 = Student {
        name: String::from("John"),
        balance: 10.0,
        have_cost: 0.0,
    };
    student1.purchase(&drink1);
    student1.purchase(&drink2);
    student1.purchase(&drink3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let point1 = Point { x: 0.0, y: 0.0 };
        let point2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(point1.distance(&point2), 5.0);
    }

    #[test]
    fn test_drink_show() {
        let drink = Drink {
            name: String::from("Coke"),
            price: 3.0,
            flavor: Flavor::NoSugar,
        };
        drink.show();
    }

    #[test]
    fn test_student_purchase() {
        let mut student = Student {
            name: String::from("John"),
            balance: 10.0,
            have_cost: 0.0,
        };
        let drink = Drink {
            name: String::from("Coke"),
            price: 3.0,
            flavor: Flavor::NoSugar,
        };
        student.purchase(&drink);
        assert_eq!(student.balance, 7.0);
        assert_eq!(student.have_cost, 3.0);
    }

    #[test]
    fn test_student_purchase_insufficient_funds() {
        let mut student = Student {
            name: String::from("John"),
            balance: 2.0,
            have_cost: 0.0,
        };
        let drink = Drink {
            name: String::from("Coke"),
            price: 3.0,
            flavor: Flavor::NoSugar,
        };
        student.purchase(&drink);
        assert_eq!(student.balance, 2.0);
        assert_eq!(student.have_cost, 0.0);
    }

    #[test]
    fn test_student_purchase_exceeds_max_cost() {
        let mut student = Student {
            name: String::from("John"),
            balance: 10.0,
            have_cost: 2.0,
        };
        let drink = Drink {
            name: String::from("Coke"),
            price: 3.0,
            flavor: Flavor::NoSugar,
        };
        student.purchase(&drink);
        assert_eq!(student.balance, 7.0);
        assert_eq!(student.have_cost, 5.0);
    }
}
