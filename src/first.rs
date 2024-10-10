pub struct List {
    // We've abstracted out the enum so as to keep implementation details private.
    head: Link,
}

struct Node {
    elem: i32,
    next: List,
}

enum Link {
    Empty,
    More(Box<Node>),
}
