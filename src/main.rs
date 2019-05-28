extern crate quick_protobuf;

pub mod example;

use example::{Subs,Views,Videos,Metrics};
use quick_protobuf::{MessageRead, BytesReader, Writer,deserialize_from_slice, serialize_into_vec};

fn main() {
    let subs = Some(Subs {
        id: vec![1,2,3,4,5,6,7,8,9,10],
        subs: vec![1,2,3,4,5,6,7,8,9,10]
    });

    let views = Some(Views {
        id: vec![1,2,3,4,5,6,7,8,9,10],
        views: vec![1,2,3,4,5,6,7,8,9,10]
    });

    let videos = Some(Videos {
        id: vec![1,2,3,4,5,6,7,8,9,10],
        videos: vec![1,2,3,4,5,6,7,8,9,10]
    });

    let metrics = Metrics {
        subs,
        views,
        videos
    };

    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&metrics)
            .expect("Cannot write message!");
    }
    println!("Message written successfully! bytes={:?}", &out);

    let read_message = {
        let mut reader = BytesReader::from_bytes(&out);
        reader
            .read_message::<Subs>(&out)
            .expect("Cannot read message")
    };

    let read_vec: Subs = deserialize_from_slice(&out).expect("Cannot write message!");
    println!("Message read back and everything matches!");
    println!("{:#?}", read_message);
}