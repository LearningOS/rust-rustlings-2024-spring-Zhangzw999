/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
            // q2的后端用于入栈
            // q1的前端用于出栈
            // 两者形成 U 形结构
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
        // 接下来需要将elem移动到栈顶
        while let Ok(val) = self.q2.dequeue() {
            self.q1.enqueue(val);
            // 经过该操作后，q2被清空。所有元素全部进入q1，elem在队头（q1）
        }
        // 交换以确保q1为空，为下一次入栈做准备
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        match self.q2.dequeue() {
            Ok(val) => Ok(val),
            Err(_) => Err("Stack is empty"),
        }
		
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.elements.is_empty() && self.q2.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}