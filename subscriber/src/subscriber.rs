
use std::str;
use std::error::Error;
use nanoid::nanoid;
use log::{error, info};
use zmq::{Context, DEALER, Socket};
use protobuf::{parse_from_bytes, RepeatedField};
use sawtooth_sdk::messages::validator::{Message, Message_MessageType};
use sawtooth_sdk::messages::client_event::{
    ClientEventsSubscribeRequest,
    ClientEventsSubscribeResponse, 
    ClientEventsSubscribeResponse_Status
};
use sawtooth_sdk::messages::events::{Event, EventFilter, EventFilter_FilterType, EventList, EventSubscription};
use crate::archer::{any_as_u8_slice, NAME as NAMESPACE};

const NULL_BLOCK_ID: &str = "0000000000000000";

pub struct Subscriber {
    subscriptions: Vec<EventSubscription>,
    event_handlers: Vec<Box<dyn Fn(Vec<Event>)>>,
    endpoint: String,
    is_active: bool,
}

impl Subscriber {
    pub fn new(endpoint: &str) -> Self {
        Subscriber {
            subscriptions: Vec::<EventSubscription>::new(),
            event_handlers: Vec::<Box<dyn Fn(Vec<Event>)>>::new(),
            endpoint: String::from(endpoint),
            is_active: false,
        }
    } 

    pub fn start(&mut self, known_ids: Option<&[String]>) -> Result<(), Box<dyn Error>> {
        let context: Context = Context::new();
        let socket: Socket = context.socket(DEALER)?;
        let mut request: ClientEventsSubscribeRequest = ClientEventsSubscribeRequest::new();
        let correlation_id: &str = &nanoid!();
        let mut message: Message = Message::new();
        let last_known_ids: Vec<String> = match known_ids {
            Some(ids) => ids.to_vec(),
            None => vec![String::from(NULL_BLOCK_ID)],
        };

        // TODO test with this URL
        socket.connect(&self.endpoint).expect("Error establishing socket connection");

        info!("Subscribing to state delta events");

        self.init_subscriptions();

        request.set_subscriptions(RepeatedField::from_vec(self.subscriptions.to_vec()));
        request.set_last_known_block_ids(RepeatedField::from_vec(last_known_ids));

        let content: &[u8] = unsafe { any_as_u8_slice(&request) };

        message.set_message_type(Message_MessageType::CLIENT_EVENTS_SUBSCRIBE_REQUEST);
        message.set_correlation_id(correlation_id.to_string());
        message.set_content(content.to_vec());

        let message_bytes: &[u8] = unsafe { any_as_u8_slice(&message) };

        info!("Sending multipart message");

        socket.send_multipart([message_bytes].iter(), zmq::SNDMORE)?;

        let mut multipart_response: Vec<Vec<u8>> = socket.recv_multipart(13)?;

        message = parse_from_bytes(&multipart_response.pop().expect("Error popping bytes from multipart response"))?;

        if message.message_type != Message_MessageType::CLIENT_EVENTS_SUBSCRIBE_RESPONSE {
            error!("Error: Unexpected message type")
        }

        let response: ClientEventsSubscribeResponse = parse_from_bytes(&message.content)?;

        if response.status != ClientEventsSubscribeResponse_Status::OK {
            error!("Subscription failed: {:?}", response.response_message)
        }

        self.is_active = true;

        info!("Successfully subscribed to state delta events!");

        self.listen(&socket)?;

        Ok(())
    }

    fn listen(&self, socket: &Socket) -> Result<(), Box<dyn Error>> {
        info!("Listening for events");
        
        while self.is_active {
            let mut multipart_response = socket.recv_multipart(13)?;
            let message: Message = parse_from_bytes(&multipart_response.pop().expect("Error popping bytes from multipart response"))?;

            if message.message_type != Message_MessageType::CLIENT_EVENTS {
                error!("Error: Unexpected message type")
            }

            let events: EventList = parse_from_bytes(&message.content)?;

            for handler in self.event_handlers.iter() {
                handler(events.get_events().to_vec());
            } 
        }
        Ok(())
    }

    fn init_subscriptions(&mut self) {
        let mut block_sub = EventSubscription::new();
        let mut delta_sub = EventSubscription::new();
        let mut delta_sub_event_filter = EventFilter::new();

        block_sub.set_event_type(String::from("sawtooth/block-commit"));
        delta_sub.set_event_type(String::from("sawtooth/state-delta"));
        
        delta_sub_event_filter.set_key(String::from("address"));
        delta_sub_event_filter.set_match_string(format!("^{}.*", NAMESPACE));
        delta_sub_event_filter.set_filter_type(EventFilter_FilterType::REGEX_ANY);
        delta_sub.set_filters(RepeatedField::from_vec(vec![delta_sub_event_filter]));

        self.subscriptions.append(&mut vec![block_sub, delta_sub]);
    }

    pub fn add_handler(&mut self, handler: Box<dyn Fn(Vec<Event>)>) {
        self.event_handlers.push(handler);
    }

    pub fn clear_handlers(&mut self) {
        self.event_handlers.clear();
    }

}