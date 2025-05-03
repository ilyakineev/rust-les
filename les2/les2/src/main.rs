fn main() {
    //Переменные
    println!("---Переменные---");
    let str = "Привет";
    println!("{}", str);

    let num: i32 = 99;
    println!("{}", num);

    //Изменяемые переменные
    println!("---Изменяемые переменные---");
    let mut str = "Привет";
    println!("{}", str);
    str = "Как дела?";
    println!("{}", str);

    let mut num: i32 = 99;
    println!("{}", num);
    num += 100;
    println!("{}", num);

    //Затемнения переменных
    println!("---Затемнения переменных---");
    let num: i32 = 99;
    println!("{}", num);
    let num = 299;
    println!("{}", num);

    //Константы
    println!("---Константы---");
    const CONST: &str = "Константы тут!!!";
    println!("{CONST}");

    //Кортеж
    println!("---Кортеж---");
    let tuple: (i32, bool, char) = (500, true, 'R');
    println!("{}",tuple.0);
    println!("{}",tuple.1);
    println!("{}",tuple.2);

    //Массив
    println!("---Массив---");
    let arr: [i32; 3] = [1,2,4];
    println!("{}",arr[1]);
}
