const subjects = ["Bio", "Inf"];

#[derive(Debug)]
struct Schüler {
    id: String,
    subjects: Vec,

}

fn main() {
    let emil = Schüler {
        id: String::from("Emil")
    };
    println!("{:?}", emil);
}
