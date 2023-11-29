use std::io;


// 키와 몸무게 데이터를 가지는 구조체 --- (*1)
struct Body {
    weight: f64,
    height: f64,
}

impl Body {
    fn new(weight: f64, height: f64) -> Body{
        Body {
            weight,
            height
        }
    }


            // BMI를 계산하는 함수 --- (*4)
            fn calc_bmi(&self) -> f64 {
                let h = self.height / 100.0;
                self.weight / h.powf(2.0)
            }


    fn show(&self) {
        println!("BMI of weight:{}, height:{} is {}", self.weight, self.height, self.calc_bmi());
}
}

//Function to get input
fn input(prompt: &str) -> f64 {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please input a valid number")
}



fn main() {
    let w = input("Write Weight:");
    let h = input("Write Height:");
    let bmi = Body::new(w,h);
    bmi.show();
}







