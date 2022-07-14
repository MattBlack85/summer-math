extern crate rand;

use std::env;

use crate::rand::{thread_rng, Rng};
use chrono::Local;
use lettre::message::{Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::Transport;

fn main() {
    let email_from = match env::var("SM_EMAIL_FROM") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_FROM env var not found"),
    };

    let email_from_alias = match env::var("SM_EMAIL_FROM_ALIAS") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_FROM_ALIAS env var not found"),
    };

    let email_to = match env::var("SM_EMAIL_TO") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_TO env var not found"),
    };

    let email_to_alias = match env::var("SM_EMAIL_TO_ALIAS") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_TO_ALIAS env var not found"),
    };

    let email_subject = match env::var("SM_EMAIL_SUBJECT") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_SUBJECT env var not found"),
    };

    let email_body = match env::var("SM_EMAIL_BODY") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_BODY env var not found"),
    };

    let email_signature = match env::var("SM_EMAIL_SIGNATURE") {
        Ok(val) => val,
        Err(_) => panic!("SM_EMAIL_SIGNATURE env var not found"),
    };

    let gmail_user = match env::var("SM_GMAIL_USERNAME") {
        Ok(val) => val,
        Err(_) => panic!("SM_GMAIL_USERNAME env var not found"),
    };

    let gmail_password = match env::var("SM_GMAIL_PASSWORD") {
        Ok(val) => val,
        Err(_) => panic!("SM_GMAIL_PASSWORD env var not found"),
    };

    let mut rng = thread_rng();
    let now = Local::now().format("%Y-%m-%d");
    let title = format!("Math excercises for {}\n\n\n\n", now);
    let mut final_file = String::from(&title);

    // Multiplications
    for _ in 1..15 {
        let first_num = rng.gen_range(10, 100);
        let second_num = rng.gen_range(10, 100);
        let op = format!("{} * {} =\n\n", first_num, second_num);
        final_file.push_str(op.as_str());
    }

    // Divisions
    for _ in 1..15 {
        loop {
            let first_num = rng.gen_range(10, 100);
            let second_num = rng.gen_range(3, 9);

            if first_num % second_num == 0 {
                let op = format!("{} : {} =\n\n", first_num, second_num);
                final_file.push_str(op.as_str());
                break;
            };
        }
    }

    let attachment = Attachment::new(format!("esercizi-{}.docx", &now)).body(
        final_file,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            .parse()
            .unwrap(),
    );

    let message = Message::builder()
        .from(
            format!("{} <{}>", email_from_alias, email_from)
                .parse()
                .unwrap(),
        )
        .reply_to(
            format!("{} <{}>", email_from_alias, email_from)
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", email_to_alias, email_to)
            .parse()
            .unwrap())
        .subject(email_subject.as_str())
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::plain(format!(
                    "{}\n\n{}",
                    email_body, email_signature
                )))
                .singlepart(attachment),
        )
        .unwrap();

    let creds = Credentials::new(gmail_user, gmail_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
