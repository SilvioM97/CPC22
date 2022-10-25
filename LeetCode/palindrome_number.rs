//Solved without transforming the number into a string
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y: i32 = x;
        let mut z:i32 = x;
        let mut i: u32 = 1;
        let mut j: u32 = 0;
        if x < 0 {  //A negative number can not be palindrome
            return false;
        }
        else if x < 10 {    //A number with one digit is trivially palindrome
            return true;
        }
        else {
            //We find the number of digits of x (i will be = to n_digit-1)
            while 10i32.pow(i+1) <= x && i < 9 { 
                i = i + 1;
            }
            //Check if the (i-j+1)-th digit is equal to the (j+1)-th one
            while 2*j < i {
                if y/(10i32.pow(i-j)) != (z%(10i32.pow(j+1)))/(10i32.pow(j)) {
                    return false;
                }
                y = y - (y/(10i32.pow(i-j)))*10i32.pow(i-j); //Remove the most significant digit
                z = z - z%(10i32.pow(j+1)); //Set the least significant digit to 0
                j = j + 1;
            }
            return true;
        }
    }
}
