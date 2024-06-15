use chrono::Datetime;
pub trait Rewindable{
    fn timestamp(&self) -> Datetime;
}


// define the stream data here

// broadcast to the kanal::broadcast
pub fn rewind_broadcast(channel: kanal::broadcast, events: impl IntoIterator<Item = impl T>){
    // set up message queue here
    let mut events:Vec<T> = events.into();
    // return early for the edge case there was no vents
    if events.is_empty() {
        return;
    }
    // TODO sort events by timestamp
    events.sort_by(|x|x.timestamp());
    // get offset
    let time_now = chrono::Utc::timestamp();
    // await per next offset up, until hte events get used up
    let time_start = eventts.first().unwrap().first();
    let offset = time_start-time_now;
    for event in events{
        // simplify await logic with the use of condvar
        // await until the timee has passed
        // simply set an await until which use the offset
        let time_next_event = event.timestamp()+time_offset;
        chrono::await_until(time_next_event)l
        self.broadcast(event)
    }
}