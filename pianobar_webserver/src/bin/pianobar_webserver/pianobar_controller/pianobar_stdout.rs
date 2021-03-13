use anyhow::{bail, Result};
use tokio::{io::AsyncReadExt, process::ChildStdout, sync::broadcast};

#[derive(Clone, Debug)]
pub enum PianobarMessage {
    UnknownMessage(String),
}

pub async fn process_pianobar_output(
    mut pianobar_stream: ChildStdout,
    pianobar_received_messages: broadcast::Sender<PianobarMessage>,
) -> Result<()> {
    loop {
        // TODO add custom message format to pianobar config,
        // parse stream into messages
        let mut output = [0u8; 100000];
        let num_read = pianobar_stream.read(&mut output).await?;
        if num_read == 0 {
            bail!("pianobar program closed!");
        }
        let msg = std::str::from_utf8(&output[..num_read])?.to_string();

        log::debug!("\n{}", msg);

        match pianobar_received_messages.send(PianobarMessage::UnknownMessage(msg)) {
            Ok(num_receivers) => {
                log::debug!("Sent pianobar message to {} listeners.", num_receivers)
            }
            Err(broadcast::error::SendError(msg)) => {
                log::error!("No receiver for message: {:?}", msg);
            }
        };
    }
}
