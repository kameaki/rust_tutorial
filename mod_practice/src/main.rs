// 同じ名前のモジュールを探してくる
mod twice;

pub use crate::twice::dance;

pub fn main(){
    dance::ok();
}