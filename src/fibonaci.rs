pub mod calculation {
    use num_bigint::BigInt;
    use num_traits::{One, Zero};
    use std::fmt::{self, Display};
    use std::mem::replace;
    use uint::construct_uint;

    construct_uint! {
        pub struct u512(8);
    }
    construct_uint! {
        pub struct u1024(16);
    }
    construct_uint! {
        pub struct u2048(32);
    }
    construct_uint! {
        pub struct u4096(64);
    }      

    pub enum MyResult {
        Above(BigInt),
        Under(Data_type),
    }

    pub enum Data_type{
        Under_u512(u512),
        Under_u1024(u1024),
        Under_u2048(u2048),
        Under_u4096(u4096),
    }

    impl Display for MyResult {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MyResult::Under(r) => write!(f, "{}", r),
                MyResult::Above(r) => write!(f, "{}", r),
            }
        }
    }

    impl Display for Data_type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data_type::Under_u512(r) => write!(f, "{}", r),
            Data_type::Under_u1024(r) => write!(f, "{}", r),
            Data_type::Under_u2048(r) => write!(f, "{}", r),
            Data_type::Under_u4096(r) => write!(f, "{}", r),
            }
        }
    }

    pub fn fibonacci(n: u128) -> MyResult {
        if n <= 739 {
            let mut sum = u512::from(0);
            let mut num1 = u512::from(1);
            for _ in 0..n {
                let temp = &num1 + &sum;
                num1 = replace(&mut sum, temp);
            }
            MyResult::Under(Data_type::Under_u512(sum))
        }
        else if n <= 1476 {
            let mut sum = u1024::from(0);
            let mut num1 = u1024::from(1);
            for _ in 0..n {
                let temp = &num1 + &sum;
                num1 = replace(&mut sum, temp);
            }
            MyResult::Under(Data_type::Under_u1024(sum))
        }
        else if n <= 2951{
            let mut sum = u2048::from(0);
            let mut num1 = u2048::from(1);
            for _ in 0..n {
                let temp = &num1 + &sum;
                num1 = replace(&mut sum, temp);
            }
            MyResult::Under(Data_type::Under_u2048(sum))
        }
        else if n <= 5091 {
            let mut sum = u4096::from(0);
            let mut num1 = u4096::from(1);
            for _ in 0..n {
                let temp = &num1 + &sum;
                num1 = replace(&mut sum, temp);
            }
            MyResult::Under(Data_type::Under_u4096(sum))
        }
        else {
            let mut sum = BigInt::zero();
            let mut num1 = BigInt::one();
            for _ in 0..n {
                let temp = &num1 + &sum;
                num1 = replace(&mut sum, temp);
            }
            MyResult::Above(sum)
            }
    }
}

