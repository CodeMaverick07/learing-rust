


fn main(){
   let nums = vec![1,2,3,4,5,6,7,8,9];

   let iter = nums.iter();

   let  filered_iter = iter.filter(|x| *x%2==0).map(|x| x*2);



 let new_vec:Vec<i32> = filered_iter.collect();

 print!("{:?}",new_vec);
  

}
