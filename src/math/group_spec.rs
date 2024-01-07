#[cfg(test)]
use num_bigint::{BigUint, ToBigUint};
use log::{info, warn, debug, error};

use rstest::rstest;

use crate::math::group::Group;

#[rstest]
#[case( BigUint::new([7].to_vec()), BigUint::new([3].to_vec()), BigUint::new([3].to_vec()), BigUint::new([6].to_vec()) ) ]
#[case( BigUint::new([7].to_vec()), BigUint::new([3].to_vec()), BigUint::new([4].to_vec()), BigUint::new([0].to_vec()) ) ]
fn add( #[case] base:BigUint, #[case]a:BigUint, #[case]b:BigUint, #[case]expected:BigUint)
{
    let a1 : Group = Group{ base : base.to_biguint().unwrap(), value : a.to_biguint().unwrap() };
    let b1 : Group = Group{ base : base.to_biguint().unwrap(), value : b.to_biguint().unwrap() };
    let result = a1.add(&b1);

    assert_eq!(result.value, expected);
}


#[rstest]
#[case( BigUint::new([7].to_vec()), BigUint::new([5].to_vec()), BigUint::new([3].to_vec()), BigUint::new([2].to_vec()) ) ]
#[case( BigUint::new([7].to_vec()), BigUint::new([3].to_vec()), BigUint::new([4].to_vec()), BigUint::new([0].to_vec()) ) ]
fn sub( #[case] base:BigUint, #[case]a:BigUint, #[case]b:BigUint, #[case]expected:BigUint)
{
    let a1 : Group = Group{ base : base.to_biguint().unwrap(), value : a.to_biguint().unwrap() };
    let b1 : Group = Group{ base : base.to_biguint().unwrap(), value : b.to_biguint().unwrap() };
    let result = a1.sub(&b1);

    assert_eq!(result.value, expected);
}

