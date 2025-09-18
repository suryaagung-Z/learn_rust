fn main() {
    println!("Hello, world!");
}

fn capitalize(s: &str) -> String {
    s.trim()
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn variabel() {
    let name = "coki";
    println!("{}", name);

    let name = "pardede";
    println!("{}", name);

    let mut name2 = "wait...";
    println!("{}", name2);

    name2 = "coki pardede";
    println!("{}", name2);
}

#[test]
fn konstanta() {
    const PI: f32 = 22.0 / 7.0;

    println!("{}", PI);
}

#[test]
fn aritmatika() {
    let (num1, num2): (i32, i32) = (10, 5);

    let tambah = num1 + num2;
    println!("{} + {} = {}", num1, num2, tambah);

    let kurang = num1 - num2;
    println!("{} - {} = {}", num1, num2, kurang);

    let kali = num1 * num2;
    println!("{} * {} = {}", num1, num2, kali);

    let bagi = num1 / num2;
    println!("{} / {} = {}", num1, num2, bagi);

    let modulus = num1 % num2;
    println!("{} % {} = {}", num1, num2, modulus);

    println!("\n==========================\n");

    let (left, right) = (15, 8);

    let res_1 = left == right;
    println!("{left} == {right} ➡ {res_1}");

    let res_2 = left != right;
    println!("{left} != {right} ➡ {res_2}");

    let res_3 = left > right;
    println!("{left} > {right} ➡ {res_3}");

    let res_4 = left < right;
    println!("{left} < {right} ➡ {res_4}");

    let res_5 = left <= right;
    println!("{left} <= {right} ➡ {res_5}");

    let res_6 = left >= right;
    println!("{left} >= {right} ➡ {res_6}");

    println!("\n==========================\n");

    let (left, right) = (15, -15);

    let res_1 = -left == right;
    let res_2 = !(left == right);
    println!("{res_1} 🤝 {res_2}");

    println!("\n==========================\n");

    let (left, right) = (true, false);
    println!("AND result \t {}", left && right);
    println!("OR result \t {}", left || right);
}

#[test]
fn kondisi() {
    let num = 10;

    if num == 12 {
        println!("num adalah 12")
    } else if num > 12 {
        println!("num diatas 12")
    } else {
        println!("num dibawah 12")
    }

    let res = if num == 12 { "num is 12" } else { "num not 12" };
    println!("{res}")
}

#[test]
fn perulangan_while() {
    let mut a = 0;
    let b = 6;

    while a < b {
        let mut c = 0;
        let d = a;

        while c <= d {
            print!("* ");
            c += 1;
        }

        println!();
        a += 1;
    }
}

#[test]
fn perulangan_loop() {
    let mut a = 0;
    let b = 6;

    loop {
        println!("{a}");
        a += 1;
        if a > b {
            break;
        }
    }

    println!("\n============= segitiga =============\n");

    let mut w = 0;
    let x = 6;

    loop {
        let mut y = x;
        let z = w;

        loop {
            print!("* ");
            y -= 1;
            if y < z {
                break;
            }
        }

        println!();

        w += 1;
        if w > x {
            break;
        }
    }

    println!("\n============= label =============\n");

    let mut i = 0;
    let max = 9;

    'mainLoop: loop {
        i += 1;
        let mut j = 0;

        loop {
            if i > max {
                break 'mainLoop;
            }

            j += 1;
            if j > i {
                break;
            }

            print!("{i} ");
        }

        println!();
    }

    println!("\n============= penambahan genap =============\n");

    let mut num = 20;
    let mut res = 0;

    let result = loop {
        if num == 0 {
            break res;
        }

        if num % 2 == 0 {
            res += num;
        }

        num -= 1;
    };
    println!("Penjumlahan bilangan genap, range 1 ➡ 20 = {result}");

    println!("\n============= For In =============\n");

    let range = 5;

    for i in 0..range {
        let sep = if i < range - 1 { " | " } else { "" };

        print!("{i}{sep}");
    }

    println!("\n============= For In =============\n");

    let range = 5;

    for i in 0..=range {
        let sep = if i < range { " | " } else { "" };

        print!("{i}{sep}");
    }

    println!("\n============= Label For In =============\n");

    'perulangan: for i in 0..range {
        if i > 3 {
            println!("Perulangan dihentikan pada langkah ke-{i}");
            break 'perulangan;
        }

        println!("{i}");
    }
}

#[test]
fn array() {
    let numbers = [1, 2, 3, 4];
    println!("Array ➡ {numbers:?}");

    println!("\n============= Array len() =============\n");
    let data_num = [0; 10];
    println!("{data_num:?}");
    println!("data_num size: {}", data_num.len());

    println!("\n============= For In Array =============\n");
    let names = ["lorem iPsum", "alI", "si alay", "jhon doe", " abc "];
    for name in names {
        println!("- {}", capitalize(name));
    }

    println!("\n============= For In  =============\n");
    for i in 0..names.len() {
        println!("{}. {}", i + 1, capitalize(names[i]));
    }
}

#[test]
fn tuple() {
    let mut tuple1: (&str, &str, i32, [i32; 2]) = ("coki", "sukiharjo", 1999, [15, 8]);
    println!("{:#?}", tuple1);

    tuple1.0 = "supriyadi";

    println!("{0}{2} {1}", tuple1.0, tuple1.2, tuple1.3[0]);

    let (name, place, year, [date, month]) = tuple1;
    println!("NAME: {}", name);
    println!("PLACE: {}", place);
    println!("YEAR: {}", year);
    println!("DATE: {}", date);
    println!("MONTH: {}", month);
}
