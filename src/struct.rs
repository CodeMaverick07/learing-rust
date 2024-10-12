fn main () {
    let user = User {
      first_name:String::from("hemant"),
      last_name:String::from("jatal"),
      age:38,
    };
    println!("{}",user.last_name);
    println!("{}",user.first_name);
    println!("{}",user.age);
  
   
  }
  
  struct User {
      first_name:String,
      last_name:String,
      age:i32,
  }
  
  