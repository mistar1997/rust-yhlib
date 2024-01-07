pub trait symmetric_key_cryptography
{
    pub fn enc(p:Vec<u8>) -> Vec<u8>;
    pub fn dec(c:Vec<u8>) -> Vec<u8>;
    pub fn key_gen(size:i32) -> Vec<u8>;
}

pub struct Misty1 
{
    key : [u8; 16]
}

pub impl symmetric_key_cryptography for Misty1
{
    pub fn enc(p:Vec<u8>) -> Vec<u8>
    {

    }

    pub fn dec(c:Vec<u8>) -> Vec<u8>
    {
        
    }

    pub fn key_gen(size:i32) -> Vec<u8>
    {
        
    }

}

