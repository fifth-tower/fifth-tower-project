use tower::tauri_web::ipc::*;
use wasm_bindgen::prelude::Closure;

pub enum FunctionId {
    Test,
    Record,
    Login,
    Script,
    CheckPoint,
}
impl Into<String> for &FunctionId {
    fn into(self) -> String {
        match *self {
            FunctionId::Test => "test".into(),
            FunctionId::Record => "record".into(),
            FunctionId::Login => "login".into(),
            FunctionId::Script => "script".into(),
            FunctionId::CheckPoint => "check_point".into(),
        }
    }
}
impl FunctionId {
    pub fn register_func(&self, func: &Closure<dyn Fn(String)>) {
        register_func(self.into(), func)
    }
    pub fn register_func_2(&self, func: &Closure<dyn Fn(String, String)>) {
        register_func_2(self.into(), func)
    }
    pub fn register_func_with_pid(&self, pid: u32, func: &Closure<dyn Fn(String)>) {
        register_func_with_pid(self.into(), pid, func)
    }
    pub fn register_func_2_with_pid(&self, pid: u32, func: &Closure<dyn Fn(String, String)>) {
        register_func_2_with_pid(self.into(), pid, func)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_flow() {
        let a = serde_json::json!({ "query":1});
        let mut a = vec![1, 2, 3, 4];
        let mut b = a.clone();
        b.reverse();
        println!("{:?}", b);
        a.drain(0..2);
        println!("{:?}", a);
    }
}
