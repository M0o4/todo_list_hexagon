pub enum AppError {
    InputContract(anyhow::Error),
    Authorization(String),
    Checkout(String),
    DomainInvariant(String), // должен быть свой тип
    OutputPort(OutputPortError),
}

pub enum OutputPortError {
    Connection(anyhow::Error),
    NotFound(anyhow::Error),
    Contract(anyhow::Error),
    Access(anyhow::Error),
    Other(anyhow::Error),
}

impl From<OutputPortError> for AppError {
    fn from(e: OutputPortError) -> Self {
        Self::OutputPort(e)
    }
}

impl From<sqlx::Error> for OutputPortError {
    fn from(e: sqlx::Error) -> Self {
        Self::Other(e.into())
    }
}
