// Shadowing is used to downgrade or updgrade the 
// mutability modifier of a variable.
pub fn mutability_downgrade() {
    println!("mutability_downgrade\n------------");
    let mut foo: i32 = 1;
    foo += 1;
    println!("{}", foo);

    let foo = foo; // We shadow the variable to downgrade it's mutability modifier

    // foo += 1; // Now this will throw an error

    println!("{}", foo);
}

pub fn type_change() {
    println!("type_change\n------------");
    let foo = "7";

    println!("{}", foo);

    let foo = 7; // We shadow the variable to change the type to int

    println!("{}", foo);
}

// This example keeps the x value from the parent scope.
pub fn scope_temporal_shadowing() {
    let x = "foo";
    {
        let x = "bar";
        println!("{}", x);
    }
    println!("{}", x);
}

// This example removes the x value from the parent scope.
pub fn scope_permanent_shadowing() {
    let mut x = "foo";
    {
        x = "bar";
        println!("{}", x);
    }
    println!("{}", x);
}
