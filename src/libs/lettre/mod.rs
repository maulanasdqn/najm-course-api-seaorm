use crate::Config;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(
	to: &str,
	subject: &str,
	body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let config = Config::new();
	let sender_email = config.smtp_email;
	let sender_name = config.smtp_name;
	let sender_password = config.smtp_password;
	let recipient_email = to;

	let email = Message::builder()
		.from(Mailbox::new(
			Some(sender_name.replace("-", " ")),
			sender_email.parse()?,
		))
		.to(recipient_email.parse()?)
		.subject(subject)
		.body(body.to_string())?;

	let smtp_credentials =
		Credentials::new(sender_email, sender_password.replace("-", " "));

	let mailer = SmtpTransport::relay("smtp.gmail.com")?
		.credentials(smtp_credentials)
		.build();

	match mailer.send(&email) {
		Ok(_) => {
			println!("Email sent successfully to {}", to);
			Ok(())
		}
		Err(e) => {
			println!("Failed to send email: {}", e);
			Err(Box::new(e))
		}
	}
}
