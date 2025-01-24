// Safety refers to preventing bugs and errors from happening
// you create reference by borrowing from the initial owner of that value
// references allow to borrow value without taking ownership
// references can be mutable or immutable
fn main() {
    // making references mutable
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;

    // if wasn't mutable, this would fail, lead to error
    *_r +=1;

    println!("Value of _x : {}", _x);

    // you can have either one mutable reference to a value
    // or multiple immutable references

    let mut account = BankAccount{
        owner: "Kris".to_string(),
        balance: 100000.5,
    };

    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(50.5);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}