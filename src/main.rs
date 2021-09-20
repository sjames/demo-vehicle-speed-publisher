use std::{fmt::Write, sync::Arc, thread};

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
    
    let participant =   ParticipantBuilder::new().create().expect("Unable to create participant");
    
    let publisher = PublisherBuilder::new().create(&participant).expect("Unable to create publisher");

    let speed_topic = TopicBuilder::<Speed>::new().create(&participant).expect("Unable to create topic");
   
    // Synchronous writer
    let mut speed_writer = WriterBuilder::new().create(&publisher, speed_topic).expect("Unable to create write");
    
    let window_position = TopicBuilder::<Position>::new().create(&participant).expect("Unable to create topic");

    let mut window_position_writer = 
        WriterBuilder::new().create(&participant, window_position).expect("Unable to create window position writer");


    let delay = std::time::Duration::from_millis(200);

    let speed: f32 = 60.0f32;
    let mut rng = rand::thread_rng();

    loop {
        thread::sleep(delay);
        let mut y: f32 = rng.gen();
        y -= 0.5;

        let speed = Arc::new(Speed::new(KilometrePerHour(speed + y), None).unwrap());
         if let Err(e) = speed_writer.write(speed) {
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
