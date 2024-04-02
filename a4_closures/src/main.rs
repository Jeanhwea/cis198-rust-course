struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    query: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    // 构造器
    fn new(query: T) -> Self {
        Cacher {
            query: query,
            value: None,
        }
    }

    // 查询接口：只缓存一次
    fn value(&mut self, val: i32) -> i32 {
        match self.value.clone() {
            Some(v) => v,
            None => {
                let v = (self.query)(val);
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
