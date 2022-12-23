//Takes in input the string of colors (as integers) and returns the number of patriotics selections
pub fn patriotics_selections(colors: Vec<i32>) -> i32 {
    let mut state: Vec<i32> = vec![0; 3]; //Array where the first cell stores the number of different (R) we can use from each
                                          //possible combination, the second cell stores the number of different (R,W) different
                                          //pairs we can use from each possible combination and the third cell stores the number of
                                          //different (R,W,G) combinations we have (i.e. the current answer)
    let mut nx: u32 = 0; //Number of X

    for color in colors {
        if color == 0 {
            state[0] += 1; //If color==R we just increase the #(R) by one
        } else if color == 1 {
            state[1] += state[0]; //If color==W we increase the number of (R,W) by the number of (R) (because for each R in each
                                  //already present combination we can make a new couple (R,W) with the new W
        } else if color == 2 {
            state[2] += state[1]; //If color==G we increase the number of (R,W,G) by the number of (R,W) for the same reason as before
        } else {
            state[2] *= 3; //Each of the already present patriotics selections can be associated to one of the 3 possible values of
                           //the new color X
            state[2] += state[1]; //For each already present combination (R,W) we can make a new combination taking the new X as G
            state[1] *= 3; //Each of the already present (R,W) selections can be associated to one of the 3 possible values of the
                           //new color X
            state[1] += state[0]; //For each already present combination (R) we can make a new combination taking the new X as W
            state[0] *= 3; //Each of the already present (R) selections can be associated to one of the 3 possible values of the
                           //new color X
            state[0] += 3_i32.pow(nx); //For each X already present we can make 3 new (R) taking the new X as R
            nx += 1; //Increase the number of X
        }
    }
    return state[2];
}
