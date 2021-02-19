use crate::events::{join_event::JoinEvent, leave_event::LeaveEvent, message_event::MessageEvent};

pub trait EventHandler {
	fn message(&self, _event: MessageEvent) {}
	fn join(&self, _event: JoinEvent) {}
	fn leave (&self, _event: LeaveEvent) {}
}