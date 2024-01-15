use rumqttc::{Client, MqttOptions, QoS};
use rust_gpiozero::*;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut mqttoptions = MqttOptions::new("garage", "192.168.0.105", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client
        .subscribe("garageport/commands", QoS::AtMostOnce)
        .unwrap();
    thread::spawn(move || {
        for _ in 0.. {
            let payload: &str = door_status_payload(is_door_closed());
            client
                .publish("garageport/status", QoS::AtLeastOnce, false, payload)
                .unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });

    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification);
    }
}

/// Will return true magnetic switch is in on (connected) position.
fn is_door_closed() -> bool {
    let button = Button::new(23);
    button.is_active()
}

fn door_status_payload(is_door_closed: bool) -> &'static str {
    if is_door_closed {
        "closed"
    } else {
        "open"
    }
}

fn pulse_led() {
    let led = LED::new(18);
    led.on();
    sleep(Duration::from_millis(500));
    led.off();
}
