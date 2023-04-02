 //pub mod BasicAlgebra{

     
 /* Suppose we would like to find the inverse of a in mod m. Lets call it a' (a.a' mod m =1). The naive approach is fixing some k in this range and then search for all a'<m  s.t: a.a'-1=km
    However, from time complexity point of view, it doesn't work properly. 
    Another approach is using Extended Euclidean algorithm to find inverse of a in mod m.
    As you may know Extended Euclidean algorithm (EEG) finds integer coefficients x and y s.t : ax + by = gcd(a, b) 
    In our case, EEG(a,m) return x , y  s.t : ax+py=gcd(a,m). We know  a and m are prime to each other, so we will have 
     ax+my=1 => It means  ax-1=-my ~ ax mod m=1. At this point we have x which is inverse of a in mod m.
    */
    pub fn extendedGCD_inverse(a:i32,m:i32) -> i32{
        if gcd(m,a)!=1 {return 0;}
        
          let mut r1=m;
          let mut  r2=a;
          if r2==0 {return 0;}
          let mut q=r1/r2; 
          let mut r=r1-q*r2;
          let mut t1=0;
          let mut t2=1;
          let mut t=t1-q*t2;
          while r!=0 {
              r1=r2;
               r2=r;
                 t1=t2;
                    t2=t;
                q=r1/r2; 
               r=r1-q*r2;
              t=t1-q*t2;
          }
          if t2<0 {
          return t2+m;
          }
          else {
          return t2;
           }
  
      }
  
      //suppose a1>b1;
      pub fn gcd(a1:i32,b1:i32) -> i32{
          if a1==0 {return b1;}
          if b1==0 {return a1;}
          if a1/b1==0 {return b1;}
          else { gcd(b1,a1%b1)}
      }

//}