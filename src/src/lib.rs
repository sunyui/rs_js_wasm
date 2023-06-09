use wasm_bindgen::prelude::*;
use web_sys::console;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use js_sys;

#[wasm_bindgen]
pub fn hello_world() {
    console::log_1(&JsValue::from_str("测试输出一些文本信息"));
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[wasm_bindgen]
pub fn send_array_to_js() -> Box<[JsValue]> {
    vec![
        JsValue::NULL,
        JsValue::UNDEFINED,
        JsValue::from_str("123"),
        JsValue::TRUE,
        JsValue::FALSE,
    ]
    .into_boxed_slice()
}


#[derive(Serialize, Deserialize)]
pub struct Obj {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<i32>>,
    pub field3: [f32; 4],
    pub field4: bool,
    pub field5: String,
}

#[wasm_bindgen]
pub fn send_obj_to_js() -> JsValue {
    let mut map = HashMap::new();
    map.insert(0, String::from("ex"));

    let obj = Obj {
        field1: map,
        field2: vec![vec![1, 2], vec![3, 4]],
        field3: [1., 2., 3., 4.],
        field4: true,
        field5: "哈哈哈".to_string(),
    };

    serde_wasm_bindgen::to_value(&obj).unwrap()
}

// 调用 JS 中的方法
#[wasm_bindgen(module = "/js2rs/index.js")]
extern "C" {
    pub type Point;

    #[wasm_bindgen(constructor)]
    fn new(x: i32, y: i32) -> Point;

    #[wasm_bindgen(method, getter)]
    fn get_x(this: &Point) -> i32;

    #[wasm_bindgen(method, getter)]
    fn get_y(this: &Point) -> i32;

    #[wasm_bindgen(method, setter)] //5
    fn set_x(this: &Point, x: i32) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Point, y: i32) -> i32;

    #[wasm_bindgen(method)]
    fn add(this: &Point, p: Point);
}

// 这个函数 JS 侧可以继续进行调用，最终会返回一个 point 对象实例
#[wasm_bindgen]
pub fn test_point() -> Point {
    let p = Point::new(10, 10);
    let p1 = Point::new(6, 3);
    p.add(p1);
    p
}

// async/await
#[wasm_bindgen]
pub async fn greet_async(name: &str) -> Result<String, JsValue> {
    let result = format!("Hello, {}!", name);
    // 模拟一个异步操作
    let promise = js_sys::Promise::resolve(&JsValue::from(result.clone()));
    let js_future = JsFuture::from(promise);
    let value = js_future.await?;
    let result = value.as_string().ok_or_else(|| JsValue::from_str("Invalid response"))?;
    Ok(result)
}
