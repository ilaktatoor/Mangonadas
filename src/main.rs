//
// creating a calcularot that inputs :
//                                      product title
//                                      list of materials and theri price
//                                      number of pruducts created with that amount of materials
//                                      how much want as profit
// rootatkali

use std::io;


//items #[derive(Debug)]
struct Items {
    name: String,
    price: f64,
}

fn main() {

    println!("Calculadora de Precio de Productos");

    let mut tEndPrice:f64 = 0.0;
    
    let mut items: Vec<Items> = Vec::new();


    //guardar nomber del producto
    println!("Nombre del producto:");
    let mut nProducto = String::new();
    io::stdin().read_line(&mut nProducto).expect("no se pudo guardar el nomnbre");
    
    //add materials an prices
    println!("-------------Add products and Prices------------");
    loop{

        //add item
        println!("Enter product name (or 'quit' to exit):");

        let mut Iname = String::new();
        io::stdin().read_line(&mut Iname).expect("Error al guardar el nombre");
        let Iname = Iname.trim();

        if Iname.to_lowercase() == "quit"{break;}
        println!("Enter Product Prices");
        let mut pPrice = String::new();
        io::stdin().read_line(&mut pPrice).expect("Error al guardar precio");
        let pPrice:f64 = match pPrice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, input valid price");
                continue;
            }
        };

        //Create item obj
        let new_item = Items{
            name: Iname.to_string(),
            price: pPrice,
        };
        
        let new_item_price = new_item.price;
        items.push(new_item);
        tEndPrice += new_item_price;
    
    }   
        

        
        println!("precio total {}", tEndPrice);
      

    //add number of ending Products
    println!("Enter number of units that can be made: ");
    let mut pices = String::new();
    io::stdin().read_line(&mut pices).expect("Error al guardar unidades");
    let pices:f64 = match pices.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            println!("error, input valid number of pieces");
            0.0
        }
    };
    //price per units
    let mut p_unit = tEndPrice/pices;
    
    //add profit margin
    println!("Enter profit margin: ");
    let mut profitMargin = String::new();
    io::stdin().read_line(&mut profitMargin).expect("Error al guardar dato");
    let profitMargin:f64 = match profitMargin.trim().parse() {
        Ok(num) => num,
         Err(_) => {
            println!("error, invalid profit margin");
            0.0
        }
    };

    //calulate profit margin
    let mut endPricePerPice = profitMargin * p_unit;

    //show end price

    print!("\n\nProduct => {}\nWith => {} as profit margin, each prucduct should cost => ${}.\nprice per unit => ${} \ntotal expences => ${}\n\n",nProducto,profitMargin, endPricePerPice , p_unit, tEndPrice );


}



