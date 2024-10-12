


fn fib (num:i32)  {
   let mut first  = 0;
   let mut second = 1;
   if num == 0 {
  return;
   }

   if num == 1 {
    return;
   }

   for _i in 1..num-1 {
       let temp = second;
       second = second + first;
       first = temp;
       print!("{} ",second);
   }

   return;
   
  
}

