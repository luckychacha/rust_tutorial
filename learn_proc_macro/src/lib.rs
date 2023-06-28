use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

macro_rules! generate_struct {
    ($name:ident, {$($field:ident: $type:tt),* $(,)?}) => {
        #[derive(Debug, Deserialize, Serialize)]
        struct $name {
            $($field: $type),*
        }
    };
}

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() {println!(\"Hello World!\"); }".parse().unwrap()
}

fn main() {
    let json_data = json!({
        "type": "object",
        "properties": {
            "id": {
                "type": "integer",
                "format": "int64"
            },
            "petId": {
                "type": "integer",
                "format": "int64"
            },
            "quantity": {
                "type": "integer",
                "format": "int32"
            },
            "shipDate": {
                "type": "string",
                "format": "date-time"
            },
            "status": {
                "type": "string",
                "description": "Order Status",
                "enum": [
                    "placed",
                    "approved",
                    "delivered"
                ]
            },
            "complete": {
                "type": "boolean",
                "default": false
            }
        },
        "xml": {
            "name": "Order"
        }
    });

    generate_struct!(
        Order,
        {
            id: i64,
            petId: i64,
            quantity: i32,
            shipDate: String,
            status: String,
            complete: bool
        }
    );
}
