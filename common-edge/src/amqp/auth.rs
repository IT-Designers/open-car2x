#[derive(Clone)]
pub enum AmqpAuth {
    Anonymous,
    Plain { username: String, password: String },
}
