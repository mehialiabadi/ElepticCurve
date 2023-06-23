In this repo, we would like to come up with an understanding of implementing some pure Elliptic Curves (ECs) on a finite field in Rust!

Let's start with some explanation of EC and how it is associated with the finite field. 

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b ~(mod ~p)$

 By this way of definition, the curve would be non-singular (does not cross over itself and has no sharp points);  The question may someone inquiry is that why we require to limit all calculations over a finite field.  Recall that the field $\mathbb{F}_p$ uses the numbers from 0 to $p - 1$, and computations end by taking the remainder on division by $p$
 
 An uncomplicated answer is that computation over the real numbers is slow and inaccurate due to round-off errors. Cryptographic applications require fast and precise arithmetic; therefore elliptic curve groups over the finite field are used in practice. 
 
 
 As it is known, new discrete points on the EC can be created by point addition, point doubling and scalar multiplication and form an abelian group with a special “point
at infinity” O as the neutral element; thus, it is expected that meets abelian properties. 

Let's define math operations that can be done on a curve:

  Addition: if R and Q are two points on the curve, we can uniquely compute R+Q.
  
  Multiplication: if R is a point on the curve and k is any arbitrary number, we can compute k*R.
  
  Point reverse: given any point R, we can take −R to be the point opposite of it.
  
  Point subtraction: if R and Q are two points on the curve, R-Q can be computed as R+(-Q).
  
  Note:  while working with EC, neither two points multiplication nor two points division are not defined.
    
  We will start with the implementation of basic operations in an elliptic curve over a finite field with a small order. Then we will extend our implementation over a finite field with a very large order ( > 64 bits) such as spec256k1. 


