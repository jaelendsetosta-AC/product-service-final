use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Corsair SABRE PRO CHAMPION SERIES Ultra-Light".to_string(),
            price: 59.99,
            description: "Experience pinpoint precision with the Corsair SABRE PRO gaming mouse, designed for esports champions with ultra-light construction and hyper-fast response.".to_string(),
            image: "/mouse.jpg".to_string()
        },
        Product {
            id: 2,
            name: "K70 CORE SE RGB Mechanical Gaming Keyboard".to_string(),
            price: 129.55,
            description: "Take your gaming to the next level with the K70 CORE SE RGB keyboard, featuring customizable backlighting and ultra-responsive mechanical keys.".to_string(),
            image: "/keyboard.jpg".to_string()
        },
        Product {
            id: 3,
            name: "VIRTUOSO RGB WIRELESS High-Fidelity Gaming Headset".to_string(),
            price: 199.99,
            description: "Immerse yourself in premium sound with the VIRTUOSO RGB WIRELESS headset, featuring high-fidelity audio, RGB lighting, and ultra-comfortable design".to_string(),
            image: "/headphones.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Corsair XENEON Flex 45".to_string(),
            price: 1499.99,
            description: "Redefine your gaming visuals with the Corsair XENEON Flex 45, a high-resolution OLED monitor with unparalleled flexibility and vivid colors.".to_string(),
            image: "/monitor.jpg".to_string()
        },
        
    ]
}