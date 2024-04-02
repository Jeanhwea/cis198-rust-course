// 简单缓存：仅缓存第一次设置的值，通过 query 闭包进行转换
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
    // 构造器 constructor
    fn new(query: F) -> Self {
        Self {
            query: query,
            value: None,
        }
    }

    // 查询接口：只缓存一次
    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut cache = Cacher::new(|x: i32| x + 1);
    println!("{:?}", cache.value(1));
    println!("{:?}", cache.value);
    println!("{:?}", cache.value(8));
    println!("{:?}", cache.value);
}
