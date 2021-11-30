use tracing::{debug, instrument};

use crate::{Connection, Frame};

/// Represents an "unknown" command. This is not a real `redis` command.
#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub fn new(key: impl ToString) -> Self {
        Self {
            command_name: key.to_string(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.command_name
    }

    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let response = Frame::Error(format!("ERR unknown command '{}'", self.command_name));
        debug!(?response);
        dst.write_frame(&response).await?;
        Ok(())
    }
}
