// Add dependencies in your Cargo.toml:
// lettre = "0.10"
// dotenv = "0.15"

extern crate dotenv;

use lettre::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() {
    // Load environment variables from a .env file
    dotenv::dotenv().ok();

    // Get email settings from environment variables
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");
    let sender_email = env::var("SENDER_EMAIL").expect("SENDER_EMAIL not set");
    let recipient_email = env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL not set");

    // Compose the email
    let email = Message::builder()
        .from(sender_email.parse().unwrap())
        .to(recipient_email.parse().unwrap())
        .subject("Hello from Rust!")
        .body("This is a test email sent from Rust.")
        .unwrap();

    // Create SMTP transport
    let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());
    let mailer = SmtpTransport::starttls_relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }
}

