mod eleptic_curve;
mod algebra;
use crate::eleptic_curve::EC::{ElepticCurve, Point};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_onCurve() {
      
        let ec = ElepticCurve{ a:2,b:2, p:17};

        assert_eq!(ec.point_onCurve(3,5), false);
    }
    #[test]
    fn test_point_double() {
       
        let ec = ElepticCurve{ a:2,b:2, p:17};

        let doublePoint=Point{pointx:3, pointy:4};
        assert_eq!(ec.point_double(2,3), doublePoint);
    }
    #[test]
    fn test_point_addition() {
        let ec = ElepticCurve{ a:2,b:2, p:17};
        let point=Point{
            pointx:1,
            pointy:2
        };
        let point1=Point{pointx:3, pointy:4};
        let point2=Point{pointx:1, pointy:-4};
        let point3=Point{pointx:6, pointy:5};

        assert_eq!(ec.point_addition(point1,point2), point3);
    }

}
