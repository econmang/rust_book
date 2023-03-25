## Characteristics of OOP:

- Objects: Packages that contain data and methods that act on that data.
- Encapsulation: Implementation details of an object aren't accesible to code using that object. Must up its public API.
- Inheritance: Mechanism whereby an object can inherit elements from another object's defn. gaining a parent object's data and behavior.
- Polymorphism: You can substitute multiple objects for each other at runtime if they share certain characteristics.

The Gang of Four defines OOP as:

OOP programs are made up of objects. An object packages both data and the procedures that operate on that data.
The procedures are typically called methods or operations.

By the above defn., Rust would be considered object oriented since its structs contain data and the impl blocks provide methods
that act on that data.

Encapsulation is represented in the use of the pub keyword allowing access to only those objs and methods we wish to make public for consumption.

Rust does not contain a method by which inheritance is represented, though.
So if it is a requirement to have inheritance to be considered an OOP lang, Rust is not one.

A limited means of sharing code or enforcing similarity is through the use of traits, but that seems to align more with interfaces
than class defn's/inheritance. In this way, it allows us to leverage trait bounds to impose constraints on what types must provide.
This is sometimes called bounded parametric polymorphism.

The state pattern is an object oriented design pattern, wherein we define a set of states a value can have internally.
States are represented by state objects, and the value's behavior changes based on the state.
The state objects share functionality utilizing traits and structs, rather than inheritance as in OOP langs.

When business requirements change, the state pattern allows us to avoid changing the code of the value holding the state or the code that uses the value. Only the code inside one of the state objects needs to change, or other state objects can be added.
