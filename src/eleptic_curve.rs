pub mod EC{
use crate::algebra::extendedGCD_inverse;

    #[derive(Debug)]
    pub struct ElepticCurve {
        pub  a:i32,
        pub  b:i32,
        pub  p:i32
        }
    
    #[derive(Debug,PartialEq)]
    pub struct Point {
        pub  pointx:i32,
        pub  pointy:i32
        }

impl ElepticCurve {

    /* Check if point(x,y) is on the curve or not */ 
    pub fn point_onCurve(&self, x1:i32,y1:i32) -> bool {
            if ((y1*y1 ) % self.p) == (x1*x1*x1+(self.a)*x1+self.b ) % self.p {
                 true
            } else {
            false
        }
    }  

    /* This function returns point(x,y)+point(x,y) */
    pub fn point_double(&self, x:i32,y:i32) -> Point{
        if self.point_onCurve(x,y) 
        {
             let lambda=((3*x*x+self.a)%self.p * extendedGCD_inverse((2*y)%self.p,self.p))%self.p;
         let newx=(lambda*lambda-2*x)%self.p;
 
         Point{pointx:newx,pointy:(lambda*(x-newx)-y) % self.p}
        }
        else {return Point{pointx:0,pointy:0}}   
     } 

    pub fn point_addition(&self,p1:Point,q1:Point) -> Point{
        if p1.pointx==q1.pointx && p1.pointy==q1.pointy
         {
            return self.point_double(p1.pointx,p1.pointy)
        }
        
            let lambda = ((q1.pointy - p1.pointy) * extendedGCD_inverse((q1.pointx - p1.pointx)%self.p, self.p))%self.p;
            let x3=(lambda*lambda-p1.pointx-q1.pointx)%self.p;
    
            Point{pointx:x3,pointy:(lambda*(p1.pointx-x3)-p1.pointy) % self.p}
        
    }

    /* 
    The following funtion takes an integer k and retuns k.point(x,y). To do multiplication in more efficient way we have implemented 
    double and algorithm.
    When it comes to an elecptic curve, we know that multiplication of two points is not defined. However, we have k.point(x,y) for arbitraty k.
    when working on eleptic curve over a prime(or cyclic?) finite feild, always there is an integer such n called order or point(x,y)
    s.t: n.point(x,y)=0
    */
    pub fn pointMulti(&self) -> Point {
        return Point{pointx:1,pointy:1};
        //Todo
    }


    /* There is no strightforward method to compute the cardinality of eleptic curve over a finite feild;however,
    Hasse's theorem states that if E is an elliptic curve over the finite field F with order q, then the cardinality of 
    E({F} _{q}) satisfies the following eqation:
    ||E({F}}_{q})|-(q+1)|\leq 2{\sqrt {q}}
    Two next functions generate all points on the curve. 
    */
    pub fn generate_allPointsNaive(&self) -> Vec<Point> {
        let a=self.a;
        let b=self.b;
        let p=self.p;
        let mut pointslist:Vec<Point>=Vec::new();

        for ix in 0..p {
         for jy in 0..p {
            if ((jy*jy ) % p) == (ix*ix*ix+(a)*ix+b ) % p {
             let point=Point{pointx:ix, pointy:jy};
              pointslist.push(point);
                            }
                         }
                    }
            return pointslist;
    }



    pub fn generate_allPoint_babyStep(&self, base:Point)->Vec<Point> {
       let mut pointlist:Vec<Point>=Vec::new();
       let x1= base.pointx;
        let y1= base.pointy;
       pointlist.push(base);

        let mut  x2=x1;
        let mut  y2=y1;
        let mut i=0;
        let mut lambda=1;
        while i<self.p {
            if x1==x2 && y1==y2 {
                lambda=((3*x1*x1+self.a)%self.p * extendedGCD_inverse((2*y1)%self.p,self.p))%self.p;
            }
            else {
                lambda = ((y2 - y1) * extendedGCD_inverse((x1 - x2)%self.p, self.p))%self.p;

            }
            let x3= (lambda*lambda-x1-x2)%self.p;
            let y3= (lambda*(x1-x3)-y1) % self.p;
            let  nextPoint=Point{pointx:x3,pointy:y3};
            pointlist.push(nextPoint);
             x2=x3;
             y2=y3;
             i=i+1;
        }
        return pointlist;
        }
    }
    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_onCurve() {
      
        let ec = ElepticCurve{ a:2,b:2, p:17};

       // assert_eq!(ec.point_onCurve(3,5), false);
        //assert_eq!(ec.point_onCurve(5,1), true);
        assert_eq!(ec.point_onCurve(7,6), true);


    }
    #[test]
    fn test_point_double() { 
       
        let ec = ElepticCurve{ a:2,b:2, p:17};

        let doublePoint=Point{pointx:5, pointy:16};

        assert_eq!(ec.point_double(7,6), doublePoint);
    }
    #[test]
    fn test_point_addition() { 
        let ec = ElepticCurve{ a:2,b:2, p:17};
        let point=Point{
            pointx:1,
            pointy:2
        };
        let point1=Point{pointx:7, pointy:6};
        let point2=Point{pointx:3, pointy:16};
        let point3=Point{pointx:9, pointy:-1};

        assert_eq!(ec.point_addition(point1,point2), point3);
    }

}



}

