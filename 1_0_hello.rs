fn main() {
    println!("Hello World!");

    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0; // `f64`
    let default_interger = 7; // `i32`

    let mut inferred_type = 12; // use next line to interfer to i64
    inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // mutable = true; // Error! type can't be change

    let mutable = true; // Ok, Variables can be overwritten with shadowing

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Print
    println!("--------Print-----------");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("Pi is roughly {0:.3}", 3.1415926);

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    println!("----------Debug-------------");
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
    println!("{1} {0} is the {actor} name.", "Slater", "Christian", actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:#?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {:#?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);

}