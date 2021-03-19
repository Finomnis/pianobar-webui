use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::cmp::min;
use tokio::{io::AsyncReadExt, process::ChildStdout, sync::broadcast};

#[derive(Clone, Debug)]
pub enum PianobarMessage {
    SongTime {
        current: u32,
        total: u32,
        paused: bool,
    },
}

struct PianobarMessageParser {
    buffer: String,
    message_time_regex: Regex,
    message_time_previous: (u32, u32),
}

fn message_start_differs_from(message: &str, expected: &str) -> bool {
    let num_chars = min(message.len(), expected.len());
    message[..num_chars] != expected[..num_chars]
}

impl PianobarMessageParser {
    pub fn new() -> PianobarMessageParser {
        PianobarMessageParser {
            buffer: String::new(),
            message_time_regex: Regex::new(r"^-?(\d+):(\d+)/(\d+):(\d+)$").unwrap(),
            message_time_previous: (0, 0),
        }
    }

    fn process_message_time(&mut self, arguments: &[String]) -> Result<PianobarMessage> {
        let parsed_arguments = self
            .message_time_regex
            .captures(arguments.get(0).ok_or(anyhow!("Not enough arguments"))?)
            .ok_or(anyhow!("Argument format does not match."))?;

        let time_left = parsed_arguments
            .get(1)
            .ok_or(anyhow!("Can't read argument 0"))?
            .as_str()
            .parse::<u32>()?
            * 60
            + parsed_arguments
                .get(2)
                .ok_or(anyhow!("Can't read argument 1"))?
                .as_str()
                .parse::<u32>()?;

        let time_total = parsed_arguments
            .get(3)
            .ok_or(anyhow!("Can't read argument 2"))?
            .as_str()
            .parse::<u32>()?
            * 60
            + parsed_arguments
                .get(4)
                .ok_or(anyhow!("Can't read argument 3"))?
                .as_str()
                .parse::<u32>()?;

        if time_left > time_total {
            bail!("Time left is larger than total time")
        }

        let time_current = time_total - time_left;

        // Compute 'paused' info
        let (prev_current, prev_total) = self.message_time_previous;
        let paused = (prev_current == time_current) && (prev_total == time_total);
        self.message_time_previous = (time_current, time_total);

        Ok(PianobarMessage::SongTime {
            current: time_current,
            total: time_total,
            paused,
        })
    }

    fn process_message(
        &mut self,
        message_type: &str,
        message_arguments: &[String],
    ) -> Result<PianobarMessage> {
        match message_type {
            "TIME" => self.process_message_time(message_arguments),
            _ => bail!("Unknown message type received: {}", message_type),
        }
    }

    fn process(&mut self, ch: char) -> Option<PianobarMessage> {
        self.buffer.push(ch);

        // Definitely reset if we encounter a newline.
        // None of our messages contain a newline.
        if ch == '\n' {
            self.buffer = String::new();
            return None;
        }

        // Reset if the message does not start with "\x1e[[#"
        if message_start_differs_from(&self.buffer, "\x1e[[#") {
            self.buffer = String::new();
            return None;
        }

        // Message is not yet over, keep reading
        if !self.buffer.ends_with("\x1e#]]") {
            return None;
        }

        // Seems like we have a message.
        // Parse it and then reset the buffer.
        let message_parts = self.buffer[1..]
            .split("\x1e")
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();
        self.buffer = String::new();

        // Make sure it can be split into at least 2 parts (e.g. 0 arguments)
        if message_parts.len() < 2 {
            log::warn!("Invalid message received.");
            return None;
        }

        // Get message type
        let message_type = {
            let message_type_part = &message_parts[0];
            if !message_type_part.starts_with("[[#")
                || !message_type_part.ends_with("#")
                || message_type_part.len() < 4
            {
                log::warn!("Invalid message received: invalid type string");
                return None;
            }
            &message_type_part[3..message_type_part.len() - 1]
        };

        // Process message
        match self.process_message(message_type, &message_parts[1..message_parts.len() - 1]) {
            Ok(message) => Some(message),
            Err(err) => {
                log::warn!("Failed to parse message: {}", err);
                None
            }
        }
    }
}

pub async fn parse_pianobar_messages(
    pianobar_stream: &mut ChildStdout,
    pianobar_received_messages: &broadcast::Sender<PianobarMessage>,
) -> Result<()> {
    let mut message_parser = PianobarMessageParser::new();
    loop {
        // Read from pianobar stdout
        let mut output = [0u8; 128];
        let num_read = pianobar_stream.read(&mut output).await?;
        if num_read == 0 {
            bail!("pianobar program closed!");
        }
        let msg = std::str::from_utf8(&output[..num_read])?;

        // Print to console, for debugging
        log::debug!("\n{}", msg);

        // Feed all characters into the message parser
        for ch in msg.chars() {
            if let Some(parsed_message) = message_parser.process(ch) {
                // The message parser found a message, send it to all listeners
                match pianobar_received_messages.send(parsed_message) {
                    Ok(num_receivers) => {
                        log::debug!("Sent pianobar message to {} listeners.", num_receivers)
                    }
                    Err(broadcast::error::SendError(msg)) => {
                        log::error!("No receiver for message: {:?}", msg);
                    }
                };
            }
        }
    }
}
