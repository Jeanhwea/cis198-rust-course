// 简单的缓存器
struct Cacher<F>
where
    F: Fn(i32) -> i32,
{
    query: F,
    value: Option<i32>,
}

impl<F> Cacher<F>
where
    F: Fn(i32) -> i32,
{
    // 构造器
    fn new(query: F) -> Self {
        Cacher {
            query: query,
            value: None,
        }
    }

    // 查询接口：只缓存一次
    fn value(&mut self, arg: i32) -> i32 {
        match self.value.clone() {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn main() {
    let mut cache = Cacher::new(|x: i32| x);
    println!("{:?}", cache.value(1));
    println!("{:?}", cache.value);
    println!("{:?}", cache.value(8));
    println!("{:?}", cache.value);
}
