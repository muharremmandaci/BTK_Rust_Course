// class benzeri yapıların kurulabilmesi için kullanılabilir. struct gibi yapılara yeni özellikler katar.
trait Animal {
    fn create(name:&'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Self {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says Hello", self.name);
    }

}

impl Animal for Cat {
    fn create(name:&'static str) -> Self {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    let my_human = Human {name: "Muharrem"};
    my_human.talk();

    let my_cat = Cat {name: "Casper"};
    my_cat.talk();

    let my_human2 = Human::create("Mahmut");
    my_human2.talk();

    let my_human3: Human = Animal::create("Veli");
    my_human3.talk();

    let my_cat2: Cat = Animal::create("Pamuk");
    my_cat2.talk();
}
