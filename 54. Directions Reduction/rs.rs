// 54. Directions Reduction -> https://www.codewars.com/kata/550f22f4d758534c1100025a

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    
    println!("initial: {:?}", arr);
    
    let mut result :Vec<Direction> = Vec::new();
    
    let mut i :usize = 1;
    
//     loop pe slice
    while i < arr.len() {
        
//         println!("i: {}", i);
        
//      if not a match save the current item in a new verctor
        match (arr[i-1], arr[i]) {
            
            (Direction::South, Direction::North) |
            (Direction::North, Direction::South) |
            (Direction::West, Direction::East) | 
            (Direction::East, Direction::West) 
              => {
                  println!("(at match) i: {}, arr.len(): {}", i, arr.len());
                  if i < arr.len()-1  {
                    i += 1;
                    }
                 },
            _ => {
                result.push(arr[i-1]);
                println!("i: {}, arr.len(): {}", i, arr.len());
                if i == arr.len()-1  {
                    result.push(arr[i]);
                }
            },
            }
        i += 1;
    }
    
    println!("processed: {:?}", result);
    
//     if new vector is equal with initial slice return the vector (solution)
    if arr == &result
        {return result;}
        
//     else recall the function for slice of the new vector
    return dir_reduc(&result);
    
}
 