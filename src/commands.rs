use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a message into a PNG file
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
    },
    /// Decode a message from a PNG file
    Decode {
        file_path: String,
        chunk_type: String,
    },
    /// Remove a message from a PNG file
    Remove {
        file_path: String,
        chunk_type: String,
    },
    /// Print all chunks from a PNG file
    Print { file_path: String },
}
