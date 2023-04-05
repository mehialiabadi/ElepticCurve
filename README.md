In this repo, we would like to come with an understanding of implemnting some pure Eleptic Curves (ECs) on a finite Ffield in Rust!

Let's  start with some explanation on EC and how it associated with the finite feild. 

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b (mod p)$

 By this way of definition, the curve would be non-singular (does not cross over itself and has no sharp points);  The question may someone inquiry is that why we require to limit all calcualations over a finite feild.  Recall that the field $\mathbb{F}_p$ uses the numbers from 0 to $p - 1$, and computations end by taking the remainder on division by $p$
 
 An uncomplicated answer is that computation over the real numbers are slow and inaccurate due to round-off error. Cryptographic applications require fast and precise arithmetic; therefor elliptic curve groups over the finite field is used in practice. 
 
 
 As it is known, new dicerete points on the EC can be created by point addition, point doubling and scalar muliplication and form an abelian group; thus, it is expected that meet abelian properties described as follows:
 
```math
\begin{itemize}
 \item Identity 
 \item Inversion 
 \item Point addition
 \item 
 \end{itemize}
 ```

  Addition : if R and Q are two points on curve, we can uniquely compute R+Q.
  
  Multiplication : if R is a point on curve and k be any arbitraty number, we an compute k*R.
  
  Point reverse : given any point R, we can take −R to be the point opposite of it.
  
  Point subtraction : if R and Q are two points on curve, R-Q can be computed as R+(-Q).
  
  Note:  while working with EC, neither two points mulitication nor two points division is not defind.
    
  We will start with implemention of basic operations in eleptic curve over a finite feild with small order. Then we will extend our implementaion over finite feild with very large order ( > 64 bits) such as spec256k1. 


