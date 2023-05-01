// Making a function to find the largest number in a vector of integers
// fn max(list: &Vec<i32>) -> i32 {

//     let mut largest = &list[0];
    
//     for i in list {
//         if i > largest {
//             largest = i;
//         }
//     }
    
//     return *largest;
// }

// Using generic to use max function for other primitives without duplicating the code for integers!

pub fn max<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
//Traits-^-------------^
    let mut largest = &list[0];
    
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    
    return *largest;
}

// Generics for structs :
#[derive(Debug)]
pub struct Point<T> {
    pub x : T,
    pub y : T,
}

// Generics in Impl blocks :
impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    pub fn get_point(&self) -> () {
        println!("{} , {}", &self.x, &self.y);
    }
}