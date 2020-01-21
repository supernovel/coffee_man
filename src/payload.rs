use rocket::request::{FromFormValue};
use rocket::http::RawStr;

#[derive(Debug, FromForm)]
pub struct Payload {
    pub text: Option<SlackText>,
    pub token: Option<String>,
    pub command: Option<String>,
    pub response_url: Option<String>,
    pub trigger_id: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
}

/// Representation of any text sent through slack
/// the text must be processed to escape specific characters
#[derive(Debug, Default)]
pub struct SlackText(pub String);

impl<'v> FromFormValue<'v> for SlackText {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<SlackText, &'v RawStr> {
        let text = form_value.to_string();
        let s = text.chars().fold(String::new(), |mut s, c| {
            match c {
                '&' => s.push_str("&amp;"),
                '<' => s.push_str("&lt;"),
                '>' => s.push_str("&gt;"),
                _ => s.push(c),
            }
            s
        });

        Ok(SlackText(s))
    }
}