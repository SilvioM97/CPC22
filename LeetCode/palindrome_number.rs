impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y: i32 = x;
        let mut z:i32 = x;
        let mut d: i32;
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        if x < 0 {
            return false;
        }
        else if x < 10 {
            return true;
        }
        else {
            while 10i32.pow(i+1) <= x && i < 9 {
                i = i + 1;
            }
            while 2*j < i {
                d = y/(10i32.pow(i-j));
                if d != (z%(10i32.pow(j+1)))/(10i32.pow(j)) {
                    return false;
                }
                y = y - d*10i32.pow(i-j);
                z = z - z%(10i32.pow(j+1));
                j = j + 1;
            }
            return true;
        }
    }
}
