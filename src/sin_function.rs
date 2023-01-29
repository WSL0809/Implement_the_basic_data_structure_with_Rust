fn sin_function(){
    let x: f32 = 2.0;
    let y = x.sin();
    println!("sin(2) is {y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        sin_function();
    }
}