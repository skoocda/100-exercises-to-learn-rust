// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

struct DropBomb {
    red_wire: bool,
    green_wire: bool,
    blue_wire: bool
}

impl DropBomb {
    fn new() -> Self {
        DropBomb {
            red_wire: true,
            green_wire: true,
            blue_wire: true
        }
    }
    fn defuse(&mut self) {
        self.red_wire = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.red_wire == true {
            panic!("BOOM");
        } else {
            println!("Bomb has been defused. Counter-terrorists win")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
