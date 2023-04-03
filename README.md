In this repo, we would like to come with an understanding of implemnting some pure Eleptic Curves (ECs) on Finite Field in Rust!
Let's kick off with some explanation on EC and why we need to be limited in Finite feild in practice.

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b (mod p)$

Basic algebraic operations in EC over a finite feild:

  Addition : if R and Q are two point on curve, we can uniquely compute R+Q 
  Multiplication : if R is a point on curve and k be any arbitraty number, we an compute k*R
  Point reverse : given any point R, we can take −R to be the point opposite of it.
  Point subtraction : if R and Q are two point on curve, R-Q can be done as R+(-Q)
    
  We will start with implemention of basic operations in eleptic curve over a finite feild with small order. Then we will extend our
implementaion over finite feild with very large order (>64 bits) such as spec256k1. 
