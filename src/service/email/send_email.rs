use lettre::{
    address::AddressError,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use super::EmailService;

impl EmailService {
    pub async fn send_mail(&self, recipient_str: String, subject: String, body: String) {
        let res: Result<Mailbox, AddressError> = recipient_str.parse();
        let recipient;
        match res {
            Ok(m) => recipient = m,
            Err(e) => {
                eprintln!(
                    "Could not parse recipient email: {}  error: {e:?}",
                    recipient_str
                );
                return;
            }
        }

        let res = Message::builder()
            .from(self.sender.as_str().parse().unwrap())
            .to(recipient)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body);
        let message = match res {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error building email message: {e:?}");
                return;
            }
        };

        let creds = Credentials::new(self.smtp_username.to_owned(), self.smtp_pwd.to_owned());

        // Open a remote connection to gmail
        let res = SmtpTransport::relay(self.smtp_host.as_str());
        let builder = match res {
            Ok(b) => b,
            Err(e) => {
                eprintln!("error relaying mailer builder: {e:?}");
                return;
            }
        };
        let mailer = builder.credentials(creds).build();

        // Send the email
        match mailer.send(&message) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => {
                eprintln!("Could not send email: {e:?}");
                return;
            }
        }
    }
}
