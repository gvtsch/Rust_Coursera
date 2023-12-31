#[derive(Debug)]
/* The code snippet #[derive(Debug)] is a Rust attribute that is used to automatically 
generate a basic implementation of the Debug trait for a struct or an enum.

The Debug trait provides a way to format a value for debugging purposes. It allows you 
to print the internal state of a struct or an enum in a human-readable format. By 
deriving the Debug trait, you don't have to manually implement the Debug trait methods 
for your struct or enum.

When you use #[derive(Debug)], the Rust compiler will automatically generate the 
implementation of the Debug trait for your struct or enum based on its fields and variants. 
This generated implementation will include the names of the fields or variants and their 
corresponding values when you print the struct or enum using the println!("{:?}", 
my_struct) or println!("{:?}", my_enum) macros. */

struct Person {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: i32,
    age: Option<u8>,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
/* In the provided code, we have a Rust struct called Person defined with several fields 
such as first_name, last_name, email, phone_number, and age. The Person struct also has 
an associated implementation block (impl) where we define methods specific to the Person
 type.

One of the methods defined for the Person struct is full_name(). This method takes a 
reference to a Person object (&self) and returns a String representing the full name of 
the person by concatenating the first_name and last_name fields using the format!() macro.

To call the full_name() method and print the full name of a Person object, you would need 
to create an instance of the Person struct and then invoke the full_name() method on that 
instance.

Here's an example of how you can call the full_name() method and print the full name: */

fn main() {
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@mail.com".to_string(),
        phone_number: 1234567890,
        age: None,
    });

    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@mail.com".to_string(),
        phone_number: 1234567890,
        age: Some(30),
        /* In Rust, the Some keyword is used to wrap a value of a certain type within an 
        Option enum variant. The Option type is used to represent the possibility of a value 
        being present (Some) or absent (None).

        In your code snippet, age: Some(30) means that the age field is assigned the value 
        Some(30). This indicates that the age field is expected to have a value of type Option<T>,
        where T is the type of the value being wrapped. In this case, the value being wrapped is 
        30, so the type of age would be Option<i32>.

        Using Some allows you to handle cases where the value is present, while None is used to 
        handle cases where the value is absent. This is useful for scenarios where a value may or 
        may not be available, and it helps prevent null pointer errors or unexpected behavior. */
    };

    let full_name = person.full_name();
    println!("Full Name: {}", full_name);

    println!("{}", person.first_name);
    
} 