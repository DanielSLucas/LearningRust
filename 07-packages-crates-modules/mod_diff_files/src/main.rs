mod front_of_house; // definie o modulo que tá na outra pasta

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}

fn main() {

}