


fn main () {
    let name = String::from("hemant");
    let len = get_str_len(name);
    println!("{}",len)
   }
   
   fn get_str_len(str: String) ->usize  {
     return  str.chars().count();
   }
   