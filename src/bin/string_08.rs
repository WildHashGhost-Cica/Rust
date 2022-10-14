fn main(){
    let s1: &str = "Hello Wildhashghost";
    let s2: String = String::from("Hello World");
    my_function(s1);
    my_function(&s2);

}

fn my_function(a: &str) -> String{
    return format!("{}", a);
}