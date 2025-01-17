#[derive(Debug, PartialEq)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    Portuguese,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting { lang: Lang::Portuguese, message: String::from("Olá WasmEdge!") };
    v.push(g);
    let g: Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
    v.push(g);
    let g: Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
    v.push(g);
    let g: Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
    v.push(g);
    let g: Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
    v.push(g);

    for e in &v {
        println!("{:?} {}", e.lang, e.message);
    }

    if let Some(chinese) = v.iter().find(|g| g.lang == Lang::Chinese) {
        println!("Chinese greeting is {}", chinese.message);
    }
}
