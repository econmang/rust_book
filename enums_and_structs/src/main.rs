// Structs are custom data types -- similar to an object in OOP
// Function associated with struct behavior are called methods

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_user(user: &User) {
    println!("Active: {}, User: {}, Email: {}, Sign-in Count: {}", user.active, user.username, user.email, user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    };
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs -- don't have any fields
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // Instantiate User directly
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("Orig user:");
    print_user(&user1);
    // Changing a User field
    user1.email = String::from("anotheremail@example.com");
    println!("\nUpdated email field of user 1:");
    print_user(&user1);

    // Instantiate User via method
    let user2 = build_user(String::from("secondaryuser123"), String::from("secondaryuser123@example.com"));
    println!("\nUser 2 created using build_user method");
    print_user(&user2);

    // Instantiate User using update syntax to copy values from existing User
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("\nUser 3 created using the update syntx method");
    print_user(&user3);

    // Tuple Struct examples
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like Struct examples
    let _subject = AlwaysEqual;

    println!("\nThe User struct utilizes `String` to ensure that the struct actually owns the attributes associated to it.");
    println!("You can also construct structs with references, but you have to manage lifetimes associated with the struct");

    let width1 = 30;
    let height1 = 50;
    let dims = (width1, height1);
    let rect = Rectangle { height: height1, width: width1 };

    // :? debug prints and :#? pretty-prints
    println!("\nrect is defined as {:#?}", rect);
    dbg!(&rect);
    println!("The area of the rectangle is (using height and width as independent vars) is {} square pixels.", area_w_h(width1, height1));
    println!("The area of the rectangle using dimensions is {} square pixels.", area_dims(dims));
    println!("The area of the rectange using the Rectangle struct is {} square pixels", area_rect(rect));
    // dbg! macro prints output to standerr
}

fn area_w_h(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_dims(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_rect(rect: Rectangle) -> u32 {
    return rect.height * rect.width;
}
