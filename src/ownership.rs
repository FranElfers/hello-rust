/* Demonstrating ownership and borrowing */
/* THIS CODE DOESNT COMPILE, was an example out off a video */

let neuromancer = Book{};

loan_to_lucy(&neuromancer); // borrow
loan_to_nia(&neuromancer); // borrow

// we are still owner of neuromancer, it is just borrowed

withdraw_book(neuromancer); // give up ownership to fn

// we are not allowed to use `neuromancer` anymore

loan_to_priya(&neuromancer); // err: neuromancer moved