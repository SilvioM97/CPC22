//Takes in input a matrix where each row represent the cumulative (wrt days) number of attractions for a city, from 0 to D days
//takes also the number of days, returns the max number of attractions one can visit
//Exploits dynamic programming matrix M[i,j]:=max number of attractions one can visit in the first i cities in j days
//The cities have an arbitrary order (specifically, the input order)
//In M[n,D] we find the solution to the problem (max #attractions one can visit in the (first) n cities in D days)
//Two rows are sufficient to compute the matrix since for the (i+1)-th row we just need the previous row
pub fn max_attractions(cities: &Vec<Vec<i32>>, days: i32) -> i32 {
    let mut row1: Vec<i32> = vec![0; days as usize + 1]; //i-th row
    let mut row2: Vec<i32> = vec![0; days as usize + 1]; //(i+1)-th row
    let mut temp_max: i32 = 0;

    //Load the first row with the #attractions one can visit in the first city each day
    row1[1..((days as usize) + 1)].copy_from_slice(&cities[0][1..((days as usize) + 1)]);

    //For each city, each day j is filled with the maximum obtained by visiting for k days the current city (for k<=j) and the
    //remaining days the previous cities
    for i in 1..cities.len() {
        for j in 1..(days as usize + 1) {
            for k in 0..j + 1 {
                if cities[i][k] + row1[j - k] > temp_max {
                    temp_max = cities[i][k] + row1[j - k];
                }
            }
            row2[j] = temp_max;
            temp_max = 0; //reset max
        }
        row1 = row2.to_vec(); //go down by 1 row
    }
    return row1[days as usize];
}

//Exponential solution ( T(n,D) = D^{n-1} )
/*pub fn max_attractions_exp (cities: &Vec<Vec<i32>>, days: i32) -> i32 {
    if days == 0 {
        return 0;
    }
    if cities.len() == 1 {
        return cities[0][days as usize - 1];
    }
    let mut points = vec![0; days as usize + 1];
    let mut max_el = 0;
    points[0] = max_attractions(&(cities[0..cities.len()-1].to_vec()), days);
    for i in 0..(days as usize) {
        points[i+1] = cities[cities.len()-1][i] + max_attractions(&(cities[0..cities.len()-1].to_vec()), days - (i as i32) - 1);
    }

    for i in 0..points.len() {
        if points[i] > max_el {
            max_el = points[i];
        }
    }

    return max_el;
}*/


//I initially tought the problem could be reduced to the Knapsack 0-1 problem
//Take an array of values, one of weights and an integer which represent the capacity of the knapsack and returns the max value
/*pub fn knapsack_problem (values: &Vec<i32>, weights: &Vec<i32>, capacity: i32) -> i32 {
    let mut row1 = vec![0; capacity as usize + 1];
    let mut row2 = vec![0; capacity as usize + 1];
    let mut max_val: i32 = 0;

    for i in 0..values.len() {
        for j in 1..((capacity as usize) + 1) {
            if weights[i] > j as i32 {
                row2[j] = row1[j];
            }
            else {
                row2[j] = max(row1[j], row1[j-weights[i] as usize] + values[i]);
                if row2[j] > max_val {
                    max_val = row2[j];
                }
            }
        }
        row1 = row2.to_vec();
    }

    return max_val;
}*/
