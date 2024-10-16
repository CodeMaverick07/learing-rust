fn main () {
    let user = User {
      first_name:String::from("hemant"),
      last_name:String::from("jatal"),
      age:38,
    };
    println!("{}",user.last_name);
    println!("{}",user.first_name);
    println!("{}",user.age);
  
    let rect1 = Rect{
      width:20,
      height:20
    };
    println!("{}",rect1.area());
    
    
  }
  
  struct User {
      first_name:String,
      last_name:String,
      age:i32,
  }
  
  struct Rect {
      width:u128,
      height:u128,
  
  }
  impl  Rect {
      fn area(&self)->u128{
      return self.height*self.width;
      }
  }
  
  
  