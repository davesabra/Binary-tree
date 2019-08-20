
#![allow(unused_variables)]
type NodeBox = Option<Box<Node>>;

use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox
}

enum TraversalMethod {
    PreOrder,
    InOrder,
    PostOrder,
    LevelOrder,
}

impl Node {
    
    fn new(s: &str) -> Node{
        Node{payload: s.to_string(), left: None, right: None}
    }

    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }
    
    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }
	
    
    fn insert(&mut self, data: &str) {
        if data < &self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
    
    pub fn traverse(&self, tr: &TraversalMethod) {
        match tr {
            &TraversalMethod::PreOrder => self.pre_order(),
            &TraversalMethod::InOrder => self.in_order(),
            &TraversalMethod::PostOrder => self.post_order(),
            &TraversalMethod::LevelOrder => self.level_order(),
        }
    }
    
    fn pre_order(&self) {                          // pre-order traversal 
        println!(" '{}' ", self.payload);
        
        if let Some(ref left) = self.left{
            left.pre_order();                      
        }
        
        if let Some(ref right) = self.right{
            right.pre_order();                     
        } 
     }       
    
    fn in_order(&self) {         // in -order traversal                 
		
        if let Some(ref left) = self.left{
            left.in_order();                      
        }
        
        println!(" '{}' ", self.payload);
        
        if let Some(ref right) = self.right{
            right.in_order();                     
        }       
    } 
    
    fn post_order(&self) {         // post -order traversal                 
		
        if let Some(ref left) = self.left{
            left.post_order();                      
        }
        
        if let Some(ref right) = self.right{
            right.post_order();                     
        } 
        
        println!(" '{}' ", self.payload);      
    }
    
	fn level_order(&self)  {
        let mut queue: VecDeque<&Node> = VecDeque::with_capacity(10);
       
        queue.push_back(self);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            
            println!(" {} ", node.payload);
            
            if let Some( ref n ) = node.left { 
				queue.push_back(n);
			}
            // sugar for;
           // match node.left {
            //    None => {}
             //   Some( ref n) => queue.push_back(n),
            //}
            
           if let Some( ref n ) = node.right { 
				queue.push_back(n);
		    }
        }
       
    } 
         
}



fn main() {
    
   // let mut root = Node::new("root");
    //root.set_left(Node::new("left"));
    //root.set_right(Node::new("right"));

    //println!("arr {:#?}", root);
    
    let mut root = Node::new("root");
        root.insert("one");
        root.insert("two");
        root.insert("four");
       
        // println!("root {:#?}", root);
    
        println!("pre-order traversal");
        root.traverse(&TraversalMethod::PreOrder);
        
        println!("in -order traversal");
        root.traverse(&TraversalMethod::InOrder);
        
        println!("post-order traversal");
        root.traverse(&TraversalMethod::PostOrder);
        
        println!("level-order traversal");
        root.traverse(&TraversalMethod::LevelOrder); //expect root one two four
              
}


