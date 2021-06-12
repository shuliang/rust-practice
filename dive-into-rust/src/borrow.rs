#[cfg(test)]
mod tests {

    #[test]
    fn test_borrow_self() {
        // 创建了一个可变的 String 类型实例
        let mut x: String = "hello".into();

        // 调用 len(&self) -> usize 函数。 self的类型是 &Self
        // x.len() 等同于 String::len(&x)
        println!("length of String {}", x.len());
        println!("length of String {}", String::len(&x));
        assert_eq!(x.len(), String::len(&x));

        // 调用fn push(&mut self, ch: char) 函数。self的类型是 &mut Self，因此它有权对字符串做修改
        // x.push('!') 等同于 String::push(&mut x, '!')
        x.push('!');
        String::push(&mut x, '!');
        println!("length of String {}", x.len());

        // 调用 fn into_bytes(self) -> Vec<u8> 函数。注意self的类型，此处发生了所有权转移
        // x.into_bytes() 等同于 String::into_bytes(x)
        let v = x.into_bytes();
        // 再次调用len()，编译失败，因为此处已经超过了 x 的生命周期
        // println!("length of String {}", x.len());
        assert_eq!(v, "hello!!".to_string().into_bytes());

        // assert!(false);
    }
}
