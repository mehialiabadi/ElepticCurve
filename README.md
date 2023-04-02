In this repo, we would like to come with an understanding of implemnting some pure Eleptic Curves (ECs) on Finite Field in Rust!
Let's kick off with some explanation on EC and why we need to be limited in Finite feild in practice.

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b (mod p)$

As we require to work in a field, all algebraic operations should satisfy field requirements:
  Point addition in field over curve: if R and Q are two point on curve, we can uniquely compute R+Q 
  Multiplication in field over curve: if R is a point on curve and k be any arbitraty number, we an compute k*R
  Point reverse in field over curve : given any point R, we can take −R to be the point opposite it.
  Point subtraction in filed over curve: if R and Q are two point on curve, R-Q means addition of  R+(-Q)
    
  We will start with implemention of basic operations in eleptic curve over a finite feild with small order. Then we will extend our
implementaion over finite feild with very large order such as spec256k1. 