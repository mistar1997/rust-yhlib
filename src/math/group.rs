use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};
use log::{info, warn, debug, error};

pub struct Group
{
    pub base : BigUint,
    pub value : BigUint
}

impl Group
{
    
    /**
     * 
     */
    pub fn add(&self, b:&Group) -> Group
    {
        let mut ret : Group = Group{ base : self.base.to_biguint().unwrap(), value : self.value.to_biguint().unwrap() };
        
        if ret.base != b.base {
            Error("base is not match! a is {} but b is {}.", self.base, b.base);
        }

        ret.value = &self.value + &b.value;

        if ret.value >= ret.base {
            ret.value = &ret.value - &ret.base;
        }

        return ret;
    }

    /**
     * 
     */
    pub fn sub(&self, b:&Group) -> Group
    {
        let mut ret : Group = Group{ base : self.base.to_biguint().unwrap(), value : self.value.to_biguint().unwrap() };
        
        if ret.base != b.base {
            error!("base is not match! a is {} but b is {}.", self.base, b.base );
        }
  
        if ret.value >= b.value {
            ret.value = &ret.value - &b.value;
        }else {
            ret.value = &ret.value + &b.value;
            ret.value = &ret.value - &ret.base;
        }

        return ret;
    }
}
