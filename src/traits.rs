// Step 1: Define the Animal trait
trait Animal {
    fn new() -> Self;
    fn talk(&self);
    fn run(&self);
}

// Step 2: Implement the Animal trait for Cat and Dog
struct Cat;

impl Animal for Cat {
    fn new() -> Self {
        Cat
    }

    fn talk(&self) {
        println!("Meow!");
    }

    fn run(&self) {
        println!("Cat is running!");
    }
}

struct Dog;

impl Animal for Dog {
    fn new() -> Self {
        Dog
    }

    fn talk(&self) {
        println!("Woof!");
    }

    fn run(&self) {
        println!("Dog is running!");
    }
}

// Step 3: Define the Pet enum
enum Pet {
    Cat(Cat),
    Dog(Dog),
}

// Step 4: Implement the Animal trait for Pet
impl Animal for Pet {
    fn new() -> Self {
        unreachable!("Pet cannot be created directly.");
    }

    fn talk(&self) {
        match self {
            Pet::Cat(cat) => cat.talk(),
            Pet::Dog(dog) => dog.talk(),
        }
    }

    fn run(&self) {
        match self {
            Pet::Cat(cat) => cat.run(),
            Pet::Dog(dog) => dog.run(),
        }
    }
}

pub fn example() {
    let misho: Cat = Animal::new();
    misho.talk();
    misho.run();

    println!("------");

    let dogo: Dog = Animal::new();
    dogo.talk();
    dogo.run();

    println!("------");

    let pets: Vec<Pet> = vec![Pet::Cat(misho), Pet::Dog(dogo)];

    pets[0].talk(); // This will call Cat's talk()
    pets[0].run();  // This will call Cat's run()
    pets[1].talk(); // This will call Dog's talk()
    pets[1].run();  // This will call Dog's run()
}
