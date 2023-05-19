// cargo run --example trait_object
trait Test {
    fn foo(&self) -> i32;
}

struct TestObj {
    id: i32,
}

impl Test for TestObj {
    fn foo(&self) -> i32 {
        self.id
    }
}

struct TestObj2 {
    uid: i32,
}

impl Test for TestObj2 {
    fn foo(&self) -> i32 {
        self.uid
    }
}

impl From<i32> for TestObj2 {
    fn from(value: i32) -> Self {
        todo!()
    }
}

fn call_foo(item: Box<dyn Test>) {
    println!("box: {}", item.foo());
}

fn call_foo_ref(item: &dyn Test) {
    println!("ref: {}", item.foo());
}

fn call_foo_static(item: &impl Test) {
    println!("static: {}", item.foo());
}

fn call_foo_generic<T: Test>(item: &T) {
    println!("generic: {}", item.foo());
}

fn main() {
    let obj = TestObj { id: 1 };
    let obj_2 = TestObj2 { uid: 2 };

    call_foo_ref(&obj);
    call_foo_ref(&obj_2);

    call_foo(Box::new(obj));
    call_foo(Box::new(obj_2));

    let obj_3 = TestObj { id: 3 };
    let obj_4 = TestObj2 { uid: 4 };

    call_foo_static(&obj_3);
    call_foo_static(&obj_4);

    println!("{}", obj_3.foo());

    call_foo_generic(&obj_3);
    call_foo_generic(&obj_4);
}
