pub struct Bicotyledon {
    pub leaf_count: u32,
    pub root: String,
    pub trunk: String
}

pub struct Monocotyledon {
    pub leaf_count: u32,
    pub root: String,
    pub trunk: String
}

pub trait Plant {
    fn new() -> Self;
    fn plantLeafCount(&self) -> String;
    fn plantTrunkType(&self) -> u32;
    fn plantRootType(&self) -> u32;
}

impl Plant for Bicotyledon {
    fn new() -> Self {
        Bicotyledon {
            leaf_count: 999, 
            // trunk: String::from("ไม่"),
            root: String::from("Wood"), 
        }
    }

    fn plantLeafCount(&self) -> u32 {
        self.leaf_count
    }

    fn plantTrunkType(&self) -> String {
        println!("This is wood furniture!");
        let result = String::from("Wood Skin");
        return result;
    }

    fn plantRootType(&self) -> String {
        println!("This is wood furniture!");
        let result = String::from("Wood Skin");
        return result;
    }
}

// impl Plant for Monocotyledon {
//     for IronFurniture {

//     fn new() -> Self {
//         IronFurniture { skin: String::from("Iron"), price: 1111 }
//     }

//     fn getSkin(&self) -> String {
//         println!("This is iron furniture!");
//         let result = String::from("Iron Skin");
//         return result;
//     }

//     fn getPrice(&self) -> u32 {
//         self.price
//     }
// }

// function main
fn main() {
    println!("\n");
    let iron: IronFurniture = Furniture::new();
    println!("skin = {} \n price = {}", iron.getSkin().to_string(), iron.getPrice());
    println!("\n===\n");
    let wood: WoodFurniture = Furniture::new();
    println!("skin = {} \n price = {}", wood.getSkin().to_string(), wood.getPrice());
}