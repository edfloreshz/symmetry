use anyhow::Result;

pub trait Synchronization {
    /// The status of the synchronization.
    type Status;
    /// The message sent to the sync provider for internal processing.
    type Message;

    /// Should handle the connection to the repository and overall synchronization to it.
    /// Afterwards it returns a status defined by the user.
    fn sync(&self) -> Result<Self::Status>;

    /// A way to manage internal logic.
    fn handle(&self, message: Self::Message) -> Result<()>;
}
