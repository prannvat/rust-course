pub fn smart_pointers(){
    //Box smart pointer
    let single_value: Box<f64> = Box::new(0.625);
    //The value tself is on the stack but the actual data is in the heap.
    let x: f64 = 0.625;

    println!("Are the values equal: {}", x == *single_value);


    let mut stack_var : i32 = 4;
    let stack_ref:&i32 = &stack_var;

    
    let heap_var: Box<i32>  = Box::new(stack_var); // copy of stack_var is being stored on the heap and heap_var(in stack) will be pointing to that memory location in the heap.
    //box pointer also owns the value it points to unlike the simple reference pointer.
    // 
    
    stack_var = 5;
    println!("The value of stack_var = {} and heap_var = {}", stack_var, heap_var);

    let point : Box<(i32, i32)> = Box::new((100,125));
    //to access the attributes of a struct that box pointer points to, we dont need to dee-ref the value, however to access the whole struct you do.
     
    box_pointer_uses();
    custom_smart_pointer();

    singly_link_list();

    reference_counting_pointer();

    doubly_linked_list();
    
}
#[derive(Debug)]
enum List{
    Cons(i32, Option<Box<List>>),
    
}
use List::{Cons};

fn box_pointer_uses(){
    let list = Cons(1, Some(Box::new
                    (Cons(2, Some(Box::new
                    (Cons(3, None)))))));

    println!("{:?}", list)
}




use std::cell::RefCell;
/*
trait Deref{
    type Target:name_of_type;
    fn deref(&self) -> &self::Target;
}
*/
use std::ops::Deref;


impl Deref for MySmartPointer{
    type Target = i32;
    fn deref(&self) -> &i32{
        &self.value
    }
}

/*
pub trait Drop{
    fn drop(&mut self);
}
*/

impl Drop for MySmartPointer{
    fn drop(&mut self) {
        println!("Dropping MySmartPointer {:?}", self.value);
    }
}


fn custom_smart_pointer() {
    let a = 50;
    let b = Box::new(a);
    println!("{}",50 == a);
    println!("{}", 50 == *b);
    // println!("{}", a == b);   can't do this because comoaring i32 with &i32

    let sptr1 = MySmartPointer::new(a);

    let sptr2 = MySmartPointer::new(*b);
    println!("{}",a == *sptr1); // *(sptr1.deref())


    drop(sptr1);
    //pointers are automatically dropped in reverse order unless explicitly called.
}


struct MySmartPointer{value:i32}

impl MySmartPointer{
    fn new(x:i32)->MySmartPointer{
        MySmartPointer {value:x}
    }
}





struct Node< T: std::fmt:: Debug + std::marker::Copy>{
    element:T,
    next : ListPointer<T>
}


struct LinkList<T : std::fmt:: Debug + std::marker::Copy>{
    head: ListPointer<T>,
}


type ListPointer<T> = Option<Box<Node<T>>>;

impl < T: std::fmt:: Debug + std::marker::Copy> LinkList<T> {
    fn create_empty_list() -> LinkList<T> {
        LinkList { head: None }
    }

    fn add(&mut self, element: T ){
        // match self.head {
        //     None => self.head = Some(Box::new(Node {element: element,  next: None})),

        //     Some(previous_head) => {
        //         let new_head = Some(Box::new(Node 
        //             {element: element, next: Some(previous_head)
        //         }));
        //         self.head = new_head;
        //     }
        // }

        // ^^^ this implementation won't work because match will transfer the ownership, we are trying to rip off the head from a data structure we don't own.

        let previous_head: ListPointer<T> = self.head.take();

        let new_head: Box<Node<T>> = Box::new(Node {
            element: element, next: previous_head
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head: ListPointer<T> = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None
        }
    }
    fn peek(&self) -> Option<T>{
        match &self.head{
            Some(head) => Some(head.element),
            None => None
        }

    }

    fn printing(&self){
        let mut list_traversal: &ListPointer<T> = &self.head;
        loop {
            match list_traversal {
                Some(node) => {
                    println!("{:?}", node.element);
                    list_traversal = &node.next;
                }
                None => break,

            }
        }
    }
}


fn singly_link_list(){

    // let list = Node{element:1, next: None};
    // let list  = Node{element: 1, next: Some(Box::new(Node {
    //     element: 2,next:Some(Box::new(Node {
    //         element: 3,next:None

    //     }))

    // }))};

   let mut list = LinkList::create_empty_list();
   let mut list2: LinkList<&str> = LinkList::create_empty_list();

   list2.add("Hello");
   list2.add("World!");

   list.add(5);

   list.peek();

   list.printing();
   list2.printing();
        
}


//Generics and deref coercion in smart pointers.

struct NewSmartPointer<T : std::fmt::Debug>{
    value : T
}

impl<T : std::fmt::Debug> NewSmartPointer<T>{
    fn new(x:T) -> NewSmartPointer<T>{
        NewSmartPointer {value: x} }
}




impl<T : std::fmt::Debug > Deref for NewSmartPointer<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T : std::fmt::Debug> Drop for NewSmartPointer<T>{
    fn drop(&mut self) {
        println!("Dropping new smart pointer object from memory {:?}", self.value);
    }
}

fn my_fn(str: &str){
    println!("The string recieved from the main is {}", str)
}

fn generics_deref_coercion(){
    let sptr_1: NewSmartPointer<&str>  = NewSmartPointer::new("Prannvat Singh");
    my_fn(&sptr_1); // here rust will look at this variable and see if it implements trait and try to deref itself.
    // &NewSmartPointer -> &String -> &str 

    let some_vec = NewSmartPointer::new(vec![1,2,3]);

    for z in &*some_vec{
        println!("The value is {}", z);
    }
}

use std::rc::{Rc, Weak};

enum NewList {
    Cons(i32, Rc<NewList>),
    Nil,
}

fn reference_counting_pointer(){
    
    let a = Rc::new(NewList::Cons(1, Rc::new(NewList::Cons(2, Rc:: new(NewList::Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    {
    let b = Rc::new(NewList::Cons(3, Rc::clone(&a)));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    let c = Rc::new(NewList::Cons(4, Rc::clone(&a)));
    println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after code block {}", Rc::strong_count(&a));
}



fn refcell_smart_pointer(){
   
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}", x1, x2);
    //^^refcell pointer gives borrowing error at runtime unlike block pointer which gives it at compile time
}


// use std::cell::RefCell;
// use std::rc::Rc;
//^^^^^^^^^^^^^^^^^^^^^^^^
//already using above


struct DList<T>{
    head:DoublyLinkedListPointer<T>,
    tail:DoublyLinkedListPointer<T>,

}
struct DNode<T> {
    element: T,
    next: DoublyLinkedListPointer<T>,
    prev: DoublyLinkedListPointer<T>,
}

type DoublyLinkedListPointer<T> = Option<Rc<RefCell<DNode<T>>>>;

impl <T: std::fmt::Display> DNode<T> {
    fn new(element: T) -> Rc<RefCell<DNode<T>>> {
        Rc::new(RefCell::new(DNode{
            element: element,
            prev: None,
            next:None,
        }))
    }
}

impl <T: std::fmt::Display> DList<T> {
    fn new() -> Self {
        DList{
            head: None,
            tail: None,
        }
    }
    fn push_front(&mut self, element: T){
        let new_head = DNode::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = DNode::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("The list is empty");
        }
        else {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take() ;
                        self.head = Some(new_head);
                        self.head.clone()
                    },
                    None => {
                        self.tail.take();
                        println!("List is empty after removal");
                        None
                    }
                }
            });
        }
    }

    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("The list is empty");
        } else {
            self.tail.take().map(|old_tail|{
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    },
                    None => {
                        self.head.take();
                        println!("List is empty after removal");
                        None
                    }
                }
            });
        }
    }

    fn print(&self){
        if self.head.is_none() {
            println!("Empty list");
        } else{
            let mut traversal = self.head.clone();
            while !traversal.is_none() {
                print!("{} ", traversal.as_ref().unwrap().borrow().element);
                traversal = traversal.unwrap().borrow().next.clone();

            }
            println!();

        }
    }


}



fn doubly_linked_list(){
    let mut d_list1: DList<i32> = DList::new();

    d_list1.push_back(23);
    d_list1.push_front(45);

    d_list1.print();

}




//Reference Cycles

// use std::rc::(Rc, Weak);
// use std::cell::{RefCell, Ref};

struct TreeNode{
    value: i32,
    parent: RefCell<Weak<TreeNode>>, //WE WANT PARENT TO HAVE OWNERSHIP OF CHILDF
    //WE WOULD LIKE FOR NODE TO BE ABLE TO MODIFY ITS CHILDREN, THUS USING REFCELL
    //DURING THE COMPUTAION, A PARENT NODE MAY CHANGE SO WE WILL WRAO THE PARENT INSIDE A REFCELL SMART POINTER.
    //NODE SHOULDN'T TRAKE OWNERSHIP OF THE PARENT BUT BE AWARE OF THE PARENT, THEREFORE USING A WEAK REFRENCE.
    children : RefCell<Vec<Rc<TreeNode>>>,
    //WE WANT THE NODE TO HAVE MULTIPLE HILDS SO WE HAVE USE THE WRAPPED THE CHIL NODE BY A VECTOR
    //WE WANT TO TRACK PARENT NODES SO A NODE IS AWARE OF ITS PARENTS.
}

fn reference_cycle(){
    
    //smart pointers are only dropped if their strong count is 1, so if we create a cycle where pointers have more than 
    // 1 strong count, there will be a memory leak.
    // However we can use weak Rc pointers, whuch allows us to set weak references to a variable//
    //meaning only a weak reference count is incremented and not the strong count thus it can be still dropped so avoids memory leak.
    // A weak reference means the ownserships is not shared with the new variabel that is referencing to the original.

    //HERE WE KEEP TRACK OF PARENTS IN TREE DATA STRUCTURE
    
    //LEAF NODE: IE NO FURTHER CHILDREN
    let leaf = Rc::new(TreeNode{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]), // NO CHILD
    });

    // PARENT WILL BE SET TO WEAK RC REF
    let branch = Rc::new(TreeNode{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), 
        //SINCE THE PARENTS ARE SUPPOSED TO TAKE OWNERSHIP OF CHILDREN, SO WE USE THE RC CLONE FUNCTION,
        //SO AS LONG AS CHILD IS  ALIVE THE PARENT IS GURANTEED TO BE ALIVE,
        //SINCE ITS STRONG COUNT WILL BE INCREMENTED BY THE BRANCH WHICH IS THE PARENT OF THE LEAF AT THE END.

    });

    
    // THIS IS A CYCLE BECAUSE THE BRANCH IS POINTING TOWARDS LEAF CHILD AND CHILD IS POINTING TOWARD BRANCH PARENT
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}