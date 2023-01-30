#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T>{
    fn new() -> Self{
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(self: &mut Self, val: T) {
        self.data.push(val);
        self.top += 1;
    }
    fn pop(self: &mut Self) -> Option<T> {
        if self.top == 0{
            return None
        }
        self.top -= 1;
        self.data.pop()
    }
}

#[test]
fn test(){
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.pop();
    println!("{:?}", stack);
}