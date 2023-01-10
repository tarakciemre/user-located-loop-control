fn main() {
    // In rust, a break statement will be automatically associated with the innermost loop.
    // https://doc.rust-lang.org/std/keyword.break.html
    
    while true {
        while true {
            break;
        }
        println!("The inner break does not exit the outer loop.");
        break;
    }
    
    'outer1: while true {
        'inner1: while true {
            println!("inner1.");
            break 'inner1;
        }
        println!("outer1.");
        break 'outer1;
    }
    
    'outer2: while true {
        'inner2: while true {
            println!("inner2.");
            break 'outer2;
        }
        println!("outer2.");
    }
    
    // https://doc.rust-lang.org/std/keyword.continue.html
    for n in 0..10 {
        if ( n % 2 == 0) {
            continue;
        }
        println!("{}", n.to_string());
    }
    
    
    'outer3: for i in 0..10 {
        print!("\nouter: {} , inner: ", i.to_string());
        'inner3: for j in 0..10 {
            if ( j % 2 == 0) {
                continue;
            }
            if ( j > i) {
                continue 'outer3;
            }
            print!("{} ", j.to_string());
        } 
    }
}

