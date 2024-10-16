
fn main(){
    let v1 = vec![1,2,3,4,5];
    
    for v1_val in v1 {
        println!("{}",v1_val)
    }
    }
    


    fn main(){
        let mut nums = vec![1,2,3,4,5];
        let v1_iter = nums.iter();
    
        for val in v1_iter {
            println!("got {val}")
        }
    
        let v1_iter_mut = nums.iter_mut();
    
        for val in v1_iter_mut {
            *val = *val + 1;        
        }
    
        println!("{:?}" ,nums);
    }


fn main(){
    let nums = vec![1,2,3];
    let mut iter = nums.iter();
 
    while let Some(val) = iter.next() {
     print!("{}",val);
    }
 }
 
 fn main(){
    let mut nums = vec![1,2,3];
    let mut iter = nums.iter_mut();
 
    let first_number = iter.next();
    let second_number = iter.next();
    let third_number = iter.next();
 
    while let Some(val) = iter.next() {
     print!("{}",val);
    }  
 }



fn main(){
    let nums = vec![1,2,3,4,5,6,7,8,9];
 
    let iter = nums.iter();
 
    let  filered_iter = iter.filter(|x| *x%2==0).map(|x| x*2);
 
 
 
  let new_vec:Vec<i32> = filered_iter.collect();
 
  print!("{:?}",new_vec);
   
 
 }
 
 
    