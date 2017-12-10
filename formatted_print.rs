fn main() {
  println!("{} days", 31);

  println!("{}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  println!("{object} {subject} {verb}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over"
  );

  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
  println!("'{number:>width$}'", number=1, width=6);
  println!("'{number:>0width$}'", number=1, width=6);
  
  //println!("My name is {0}, {1} {0}", "Bond");  // check the number of arguments
  println!("My name is {0}, {1} {0}", "Bond", "James");
}

