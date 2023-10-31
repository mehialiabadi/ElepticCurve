In this repository, we aim to explore the implementation of pure Elliptic Curves (ECs) on a finite field in Rust!

Let's start with some explanation of EC and how it is associated with the finite field. 

The elliptic curve equation over the finite field $\mathbb{F}_p$ takes the following form:
$y^2 ≡ x^3 + ax + b ~(mod ~p)$

 
By defining the curve in this manner, we ensure it remains non-singular, avoiding any self-intersections or sharp points. You might wonder why we insist on performing all calculations within a finite field. It's important to note that the field $\mathbb{F}_p$ consists of numbers from 0 to $p - 1$, and computations are concluded by finding the remainder upon division by $p$.

The straightforward reason for this choice is that performing calculations with real numbers is both slow and susceptible to inaccuracies caused by round-off errors. Cryptographic applications demand swift and precise arithmetic, which is why elliptic curve groups over finite fields are commonly employed in practice. 
 
 
 As it is known, new discrete points on the Elliptic Curve (EC) can be generated through point addition, point doubling, and scalar multiplication. They collectively form an abelian group, incorporating a special "point at infinity" denoted as O, which serves as the neutral element. Consequently, this group is expected to exhibit abelian properties.

Now, let's establish mathematical operations that can be performed on this curve:

  Addition: if R and Q are two points on the curve, we can uniquely compute R+Q.
  
  Multiplication: if R is a point on the curve and k is any arbitrary number, we can compute k*R.
  
  Point reverse: given any point R, we can take −R to be the point opposite of it.
  
  Point subtraction: if R and Q are two points on the curve, R-Q can be computed as R+(-Q).
  
  Note:  while working with EC, neither multiplication nor division of two points is not defined.
    
  Our initial focus will be on implementing fundamental operations within an elliptic curve over a finite field with a relatively small order. Subsequently, we'll expand our implementation to encompass a finite field with a larger order. 
 
Change the value of a, b, and p parameters and enjoy making different curves (integers up to  less than 4294967295)!
The default integer type in Rust is i32. Signed integer types in Rust start with i and it has 8, 16, 32, 64, and 128-bit. The minimum and maximum values are from $- 2^{n-1} to $ 2^{n-1}-1 $.

Computation in a group with $ p \less 2^{127}-1 $ is straightforward, all that needs to be done is mod p. 
However, when the value of $ p > 2^{127} $, it required more effort. For example, consider curve speck256  used in Bitcoin's public-key cryptography. The curve is defined by the following parameters:

p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
= $2^{256} - 2^{32} - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1$

$ E: y^2 = x^3+ax+b$ over $F_p$ 

a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000

b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007

As we delve into handling substantial numbers, particularly in the case of Speck256 with a field size of 256 bits, we can create a new data structure, U256, to store field elements. With this structure, we will need to implement essential operations such as addition (+), multiplication (*), subtraction (-), and other relevant operations when working with two field elements of the U256 type.


struct U256	{i128,i128} or

struct U256 {i64,i64,i64,i64}

# run
cargo run
# test
cargo test

