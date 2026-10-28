// rust code for Alaba Electronics

// creating structure
struct Laptop {brand:String, price:u32}

impl Laptop{
    fn cost(&self, qty :u32)->u32{
        self.price * qty
    }
}

fn main() {
    //instantialization of laptop brands
    let hp = Laptop{brand:"HP".to_string(), price:650_000};
    let ibm = Laptop{brand:"IBM".to_string(), price:755_000};
    let toshiba = Laptop{brand:"Toshiba".to_string(), price:550_000};
    let dell =  Laptop{brand:"Dell".to_string(), price:850_000};

    //getting cost of 3 of each laptop brand
    let qty: u32 = 3;
    let cost1 = hp.cost(qty);
    let cost2 = ibm.cost(qty);
    let cost3 = toshiba.cost(qty);
    let cost4 = dell.cost(qty);

    //giving output to the customer
    let total_cost = cost1 + cost2 + cost3 + cost4;
    println!("The total cost of 3 laptops from each brand is ₦{}", total_cost);
}