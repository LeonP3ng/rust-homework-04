
fn main() {
    /* 第一题 */ 
    let redlight = Light{
        light_type : String::from("Red"),
        light_time : 15,
    };
    let greenlight = Light{
        light_type : String::from("Green"),
        light_time : 60,
    };
    let yellowlight = Light{
        light_type : String::from("Yellow"),
        light_time : 3,
    };
    redlight.work_time();
    greenlight.work_time();
    yellowlight.work_time();


    
    /*第二题*/
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    try_sum(&a);

    

    /* 第三题 */ 
    let trai = Traingle{
        length: 5.0,
        height: 7.0,
    };
    let rec = Rectangle{
        length: 7.0,
        width: 10.0,
    };
    let cir = Circle{
        radius: 2.0,
    };
    calculate(&trai);
    calculate(&rec);
    calculate(&cir);
}


/******第一题开始******/
pub trait TrafficLight{
    fn work_time(&self);
}

pub struct Light {
    light_type : String,
    light_time : u32,
}

impl TrafficLight for Light {
    fn work_time(&self){
        println!("{} light is : {}s", self.light_type, self.light_time);
    }
}
/******第一题结束******/


/******第二题开始******/
fn get_sum(_arr: &[u32]) -> Option<u32> {
    let mut total:u32 = 0;
    //溢出标志
    let mut ifflag = 0;
    for num in _arr{
        //对上界和下界进行判断
        if total + num <= std::u32::MAX && total + num >= std::u32::MIN {
            total = total + num;
        } else {
            ifflag = 1;
            break;
        }
    }
    if ifflag == 0 {
        Some(total)
    } else {
        None
    }
    
}

fn try_sum(_arr:&[u32]) {
    match get_sum(&_arr) {
        None => println!("溢出!"),
        Some(ans) => {
            println!("{}", ans)
        },
    }
}

/******第二题结束******/
 


/******第三题开始******/
struct Traingle{
    length: f64,
    height: f64,
}

struct Rectangle{
    length: f64,
    width: f64,
}
struct Circle{
    radius: f64,
}

pub trait PolygonTrait {
    fn getArea(&self) -> f64;
}

impl PolygonTrait for Rectangle {
    fn getArea(&self) -> f64 {
        self.length * self.width
    }
}

impl PolygonTrait for Traingle {
    fn getArea(&self) -> f64 {
        self.length* self.height * 0.5
    }
}

impl PolygonTrait for Circle {
    fn getArea(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}


pub fn calculate<T: PolygonTrait>(p: &T) {
    println!("area is :{}", p.getArea())
}
/******第三题结束******/