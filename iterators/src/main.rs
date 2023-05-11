#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
    style: String,
}

fn filter_shoe_size(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
    return shoes.into_iter().filter(|x| x.size == shoe_size).collect();
}


#[test]
fn iterator_demo() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


fn main() {

    // Iterators in rust are Lazy!
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got : {}", val);
    // }

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    
    let collection: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
    println!("{:?}", collection);

    let shoes = vec![
        Shoe {
            size : 9,
            style : String::from("Party"),
        }, 
        Shoe {
            size : 10,
            style : String::from("Formal"),
        }, 
        Shoe {
            size : 7,
            style : String::from("Formal"),
        }
    ];

    let f_shoes = filter_shoe_size(shoes, 10);

    println!("{:?}", f_shoes);
}