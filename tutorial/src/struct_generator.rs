use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// A macro rules to auto generate struct from json.
// json contains: properties and xml.
// We can get the struct name from xml.name.
// We can get the struct fields from properties.Each property key is the field name, and attribute type is the field type.
// We can get the struct attributes from properties.Each property key is the field name, and attribute type is the field type.
macro_rules! generate_struct {
    ($struct_name: ident, $($field_name: ident, $field_type: ident),*) => {
        #[derive(Serialize, Deserialize, Debug)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }
    };
}

pub fn generate() {
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

    let struct_name = json_data["xml"]["name"].as_str().unwrap();
    let properties = json_data["properties"].as_object().unwrap();
    let mut fields = Vec::new();
    let mut attributes = Vec::new();
    for (key, value) in properties {
        let field_name = key;
        let field_type = value["type"].as_str().unwrap();
        fields.push((field_name, field_type));
        let attribute_name = key;
        let attribute_type = value["type"].as_str().unwrap();
        attributes.push((attribute_name, attribute_type));
    }
    println!("struct_name: {}", struct_name);
    println!("fields: {:?}", fields);
    println!("attributes: {:?}", attributes);
    // use generate_struct to generate struct.
    generate_struct!(
        Order, id, i64, petId, i64, quantity, i32, shipDate, String, status, String, complete, bool
    );
    // create the generated struct and debug print it.
    let order = Order {
        id: 1,
        petId: 1,
        quantity: 1,
        shipDate: "2021-01-01".to_string(),
        status: "placed".to_string(),
        complete: false,
    };
    println!("{:#?}", order);
}
