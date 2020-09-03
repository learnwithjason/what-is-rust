#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Person {
  name: &'static str,
  age: u8,
  color: Color,
}

impl Person {
  fn say_hello(&self) -> String {
    format!("Hi! My name is {}.", self.name)
  }

  fn beverage(&self) -> String {
    if self.age < 21 {
      "Have some milk.".to_string()
    } else {
      "Have a cocktail.".to_string()
    }
  }
}

fn main() {
  /*
    // println!("Hello chat!");

    // let age = 10.0;
    // let color = "chartreuse";
    // let dec = 1.05;

    // println!("My age is {} and my favorite color is {}", age, color);

    // let thing = age * dec;

    // println!("{:?}", thing);

    // dbg!(thing);
  */

  let rgb = Color(255, 127, 0);
  dbg!(rgb.1);

  // let person: (&str, u8, (u8, u8, u8)) = ("Jason", 114, (255, 0, 0));
  let person = Person{ name: "Jason", age: 20, color: Color(255, 0, 0) };
  println!("{:?}",person);

  let what_i_said = person.say_hello();
  let bev = person.beverage();

  println!("{} {}", what_i_said, bev);
}
