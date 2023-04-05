In this repo, we would like to come with an understanding of implemnting some pure Eleptic Curves (ECs) on Finite Field in Rust!

Let's  start with some explanation on EC and how it associated with finite feild. 

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b (mod p)$

 By this way of definition, the curve would be non-singular (does not cross over itself and has no sharp points) 
 As it is known, new  dicerete points on the EC can be created by point addition, point doubling and scalar muliplication.  
 


As it is known that an EC on finite feild is a  Basic algebraic operations in EC over a finite feild:

  Addition : if R and Q are two points on curve, we can uniquely compute R+Q.
  
  Multiplication : if R is a point on curve and k be any arbitraty number, we an compute k*R.
  
  Point reverse : given any point R, we can take −R to be the point opposite of it.
  
  Point subtraction : if R and Q are two points on curve, R-Q can be computed as R+(-Q).
  
  Note:  neither two points mulitication nor two points divition is not defined in EC.
    
  We will start with implemention of basic operations in eleptic curve over a finite feild with small order. Then we will extend our
implementaion over finite feild with very large order (>64 bits) such as spec256k1. 
