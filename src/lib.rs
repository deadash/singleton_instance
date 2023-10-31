/// Trait for providing custom initialization for the struct.
///
/// Implement this trait to specify how the singleton instance should be initialized.
pub trait Initializable {
    /// Method for creating and initializing a new instance of the struct.
    fn initialize() -> Self;
}