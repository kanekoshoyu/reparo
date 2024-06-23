use chrono::NaiveDateTime;
use kanal::AsyncSender;
use std::{
    fs::File,
    io::{Read, Write},
};
use tokio::time::{sleep_until, Duration, Instant};

/// any event that is to be rewinded should be rewindable
pub trait Rewindable: serde::Serialize + for<'d> serde::Deserialize<'d> {
    fn timestamp(&self) -> chrono::NaiveDateTime;
}

pub fn save_vec_to_file<T: Rewindable>(vec: &Vec<T>, file_path: &str) -> std::io::Result<()> {
    let json_string = serde_json::to_string(vec)?;
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn load_vec_from_file<T: Rewindable>(file_path: &str) -> std::io::Result<Vec<T>> {
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let vec = serde_json::from_str(&json_string)?;
    Ok(vec)
}

/// pass iterator of the rewindable, and send to kanal::channel
pub async fn rewind_broadcast<T: Rewindable>(
    channel: AsyncSender<T>,
    events: impl IntoIterator<Item = T>,
) {
    // set up message queue here
    let mut events: Vec<_> = events.into_iter().collect();
    // return early for the edge case there was no vents
    if events.is_empty() {
        return;
    }
    // TODO sort events by timestamp
    events.sort_by_key(|x| x.timestamp());
    // get offset
    let time_now = chrono::Utc::now().naive_utc();
    // await per next offset up, until the events get used up
    let first_rewindable_data = events.first().unwrap();
    let time_offset = first_rewindable_data.timestamp() - time_now;
    for event in events {
        // simplify await logic with the use of condvar
        // await until the timee has passed
        // simply set an await until which use the offset
        let time_next_event = event.timestamp() + time_offset;
        wait_until_timestamp(time_next_event).await;
        let result = channel.try_send(event);
        if let Err(e) = result {
            println!("failed sending on the channel, {e}");
        }
    }
}

pub async fn wait_until_timestamp(time_target: NaiveDateTime) {
    let time_now = chrono::Utc::now().naive_utc();
    if time_target > time_now {
        let wait_duration = time_target - time_now;
        let wait_duration = Duration::from_micros(wait_duration.num_microseconds().unwrap() as u64);
        let wait_until = Instant::now() + wait_duration;
        sleep_until(wait_until).await;
    }
}

// TODO implement subscribe and store into file

// TODO implement parse file unit test

mod tests {
    /// verify that the value can be read/write
    #[tokio::test]
    async fn test_write_read() {
        // set up cache connection
        struct TestStruct {
            data: String,
            timestamp: chrono::DateTime<chrono::Utc>,
        }
        let test_struct = TestStruct {
            data: "Test".to_string(),
            timestamp: chrono::Utc::now(),
        };
        //serialize
    }
}
