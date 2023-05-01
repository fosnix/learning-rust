mod generic;
use generic::Point;
use generic::max;


fn main() {

    let list  = vec!['u', 'y', 'a', 'd'];
    println!("{}", max(&list));
    println!("{:?}", list);

    let point_1 = Point{x : 22.2, y : 33.6};
    let point_2 = Point{x : 10, y : 20};

    // This function is only available for functions for floating types ->
    point_1.get_point();

    println!("{:?}", point_1.get_x());
    println!("{:?}", point_2.get_y());
    
}  