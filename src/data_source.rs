pub struct DataPoint {
    pub item_name: String,
    pub quantity: f32,
    pub value: f32
}

pub fn get_data() -> Vec<DataPoint> {
    vec![DataPoint{
        item_name: "item1".to_string(),
        quantity: 105.0,
        value: 0.0
    },DataPoint{
        item_name: "item1".to_string(),
        quantity: 10.0,
        value: 30.0
    },
    DataPoint{
        item_name: "item2".to_string(),
        quantity: 5.0,
        value: -5.0
    },
    DataPoint{
        item_name: "item2".to_string(),
        quantity: 25.0,
        value: 105.0
    }
    ]
}