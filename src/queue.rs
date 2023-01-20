#[derive(Debug)]
struct Queue<T>{
    capacity: usize,
    data:Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            capacity: size,
            data: Vec::with_capacity(size),
        }
    }

    fn size (&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        Self::size(&self) == 0
    }

    fn enqueue(&mut self, val: T) -> Result<(),String> {
        if Self::size(&self) == self.capacity{
            return Err("No Space Available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0{
            self.data.pop()
        }else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);

        if let Err(e) = q.enqueue(4){
            println!("{}", e);
        }

        if let Some (data) = q.dequeue () {
            println!("data: {data}");
        }else {
            println!("empty queue.");
        }

        println!("size:{}, empty:{}", q.size(), q.is_empty ());
        println!("content: {:?}", q) ;
    }
}