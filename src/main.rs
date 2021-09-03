use std::{sync::Arc, thread};

use cyclonedds_rs::*;
use rand::prelude::*;
use vehicle_signals::{
    units::{KilometrePerHour, Percent},
    v2::vehicle::cabin::door::window::Position,
    v2::vehicle::Speed,
};

use cyclonedds_sys::{*};

fn main() {
    println!("Publishing vehicle speed every 10ms");
    
    let participant = DdsParticipant::create(None, None, None).unwrap();
    

    
    let publisher =
        DdsPublisher::create(&participant, None, None).expect("Unable to create publisher");

    let topic = Speed::create_topic(&participant, None, None, None).unwrap();
    let mut writer = DdsWriter::create(&publisher, topic, None, None).unwrap();

    let window_position = Position::create_topic(&participant, None, None, None).unwrap();
    let mut window_position_writer =
        DdsWriter::create(&publisher, window_position, None, None).unwrap();

    let delay = std::time::Duration::from_millis(200);

    let speed: f32 = 60.0f32;
    let mut rng = rand::thread_rng();

    loop {
        thread::sleep(delay);
        let mut y: f32 = rng.gen();
        y -= 0.5;

        let speed = Arc::new(Speed::new(KilometrePerHour(speed + y), None).unwrap());
         if let Err(e) = writer.write(speed) {
             println!("write failed:{}",e);
         }

        let pos = Position::new(Percent(25), None, 0, vehicle_signals::v2::Side::Left).unwrap();
        if let Err(e) = window_position_writer.write(Arc::new(pos)) {
            println!("write failed:{}", e);
        }

        let pos = Position::new(Percent(35), None, 0, vehicle_signals::v2::Side::Right).unwrap();
        if let Err(e) = window_position_writer.write(Arc::new(pos)) {
            println!("write failed:{}", e);
        }

        let pos = Position::new(Percent(0), None, 1, vehicle_signals::v2::Side::Left).unwrap();
        if let Err(e) = window_position_writer.write(Arc::new(pos)) {
            println!("write failed:{}", e);
        }

        let pos = Position::new(Percent(100), None, 1, vehicle_signals::v2::Side::Right).unwrap();
        if let Err(e) = window_position_writer.write(Arc::new(pos)) {
            println!("write failed:{}", e);
        }
    }
}
