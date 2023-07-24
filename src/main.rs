use std::ops::Add;

#[derive(Debug)]
enum MultipleTypes {
    Struct {
        field: i32,
    },
    Number(i32),
    String(String),
    Tuple(i32, i32),
    Unit,
    I32Array([i32; 3]),
}

impl MultipleTypes {
    fn to_string(&self) -> String {
        match self {
            MultipleTypes::Struct { field } => format!("Struct {{ field: {} }}", field),
            MultipleTypes::Number(number) => format!("Number({})", number),
            MultipleTypes::String(string) => format!("String({})", string),
            MultipleTypes::Tuple(a, b) => format!("Tuple({}, {})", a, b),
            MultipleTypes::Unit => format!("Unit"),
            MultipleTypes::I32Array(array) => format!("I32Array({:?})", array),
        }
    }
}

trait MultipleTypesTrait {
    fn to_string_trait(&self) -> String;
}

impl MultipleTypesTrait for MultipleTypes {
    fn to_string_trait(&self) -> String {
        self.to_string()
    }
}


#[derive(Debug)]
struct Addend<T> {
    a: T,
    b: T,
}

impl<T: Add<Output = T>> Add for Addend<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            a: self.a + other.b,
            b: self.b + other.a,
        }
    }
}

fn trait_add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}


fn main() {
    // iterate over enum
    println!("{}", "=====iterate over enum=====");
    let multiple_types_container = vec![
        MultipleTypes::Struct { field: 1 },
        MultipleTypes::Number(2),
        MultipleTypes::String(String::from("3")),
        MultipleTypes::Tuple(4, 5),
        MultipleTypes::Unit,
        MultipleTypes::I32Array([6, 7, 8]),
    ];
    // 类型本身实现的to_string方法
    for item in &multiple_types_container {
        println!("{:?}", item.to_string());
    }
    // iterate over trait
    println!("{}", "=====iterate over trait=====");
    // trait实现的to_string方法
    for item in &multiple_types_container {
        println!("{:?}", MultipleTypesTrait::to_string_trait(item));
        // println!("{:?}", item.to_string_trait());
    }
    // Add trait
    println!("{}", "======Add trait=====");
    let addend = Addend { a: 1, b: 2 };
    let addend2 = Addend { a: 3, b: 4 };
    println!("{:?}", addend + addend2);
    //
    let addend = Addend { a: 5, b: 6 };
    let addend2 = Addend { a: 7, b: 8 };
    let res = Add::add(addend, addend2);
    println!("{:?}", res);
    // trait_add
    let addend = Addend { a: 9, b: 10 };
    let addend2 = Addend { a: 11, b: 12 };
    let res = trait_add(addend, addend2);
    println!("{:?}", res);
}
