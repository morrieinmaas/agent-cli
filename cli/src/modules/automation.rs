use crate::copy;
use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::modules::connection::invite_url_to_object;
use crate::utils::loader::{Loader, LoaderVariant};
use crate::utils::qr;
use agent::modules::connection::{ConnectionCreateInvitationOptions, ConnectionModule};
use agent::modules::credential::CredentialModule;
use agent::modules::credential_definition::CredentialDefinitionModule;
use agent::modules::schema::SchemaModule;
use automations::automations::credential_offer::CredentialOfferAutomation;
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, trace};
use std::collections::HashMap;

#[derive(Args)]
#[clap(about = HelpStrings::Automation)]
pub struct AutomationOptions {
    #[clap(subcommand)]
    pub commands: AutomationSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum AutomationSubcommands {
    #[clap(about = HelpStrings::AutomationCredentialOffer )]
    CredentialOffer {
        #[clap(long, short, help = HelpStrings::AutomationCredentialOfferConnectionId)]
        connection_id: Option<String>,

        #[clap(long, short, default_value = "60", help = HelpStrings::AutomationCredentialOfferTimeout)]
        timeout: u32,

        #[clap(long = "self", short = 's', help = HelpStrings::AutomationCredentialOfferSelf)]
        sent_to_self: bool,

        #[clap(long, short, help = HelpStrings::AutomationCredentialOfferNoQr )]
        no_qr: bool,
    },
}

pub async fn parse_automation_args(
    options: &AutomationOptions,
    agent: impl ConnectionModule + CredentialModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    let loader = Loader::start(LoaderVariant::default());

    match &options.commands {
        AutomationSubcommands::CredentialOffer {
            connection_id,
            timeout,
            sent_to_self,
            no_qr,
        } => match connection_id {
            Some(c) => credential_offer(c.to_owned(), agent).await?,
            None => {
                let connection = agent
                    .create_invitation(ConnectionCreateInvitationOptions {
                        auto_accept: true,
                        alias: Some(String::from("automation")),
                        ..Default::default()
                    })
                    .await?;
                if *sent_to_self {
                    let invitation_object = invite_url_to_object(connection.invitation_url)?;
                    agent.receive_invitation(invitation_object).await?;
                } else {
                    if !no_qr {
                        qr::print_qr_code(&connection.invitation_url)?;
                    }
                    println!();
                    println!();
                    println!("================");
                    println!("{}", "Credential offer".bold());
                    println!("================");
                    println!();

                    println!(
                        "{} invitation with connection id {}.",
                        "Created".green(),
                        connection.connection_id.bold()
                    );
                    println!();
                    println!(
                        "{} the QR code to accept the invitation or use this URL:\n\n{}",
                        "Scan".bold(),
                        connection.invitation_url
                    );
                    println!();
                    println!();
                    println!(
                        "{} for the invitation to be accepted. Timeout is {} seconds...",
                        "Waiting".cyan(),
                        timeout
                    );
                    copy!("{}", connection.invitation_url);
                }
                debug!("Looping {} times", timeout);
                for i in 1..=*timeout {
                    let connection =
                        ConnectionModule::get_by_id(&agent, connection.connection_id.to_owned())
                            .await?;
                    if connection.state != "active" && connection.state != "response" {
                        trace!(
                            "Connection state is not active, waiting 1 second then trying again..."
                        );
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                    } else {
                        println!("Invitation {}!", "accepted".green());
                        credential_offer(connection.connection_id, agent).await?;
                        break;
                    }
                    if i == *timeout {
                        return Err(Error::InactiveConnection.into());
                    }
                }
            }
        },
    };
    println!("{} executed automation", "Successfully".green());
    println!(
        "{}: It might take a few seconds for the credential to arrive",
        "Note".cyan()
    );
    loader.stop();
    Ok(())
}

async fn credential_offer(
    connection_id: String,
    agent: impl ConnectionModule + CredentialModule + SchemaModule + CredentialDefinitionModule,
) -> Result<()> {
    // Mock credential
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert(String::from("Name"), String::from("Joyce Brown"));
    attributes.insert(String::from("Date Of Birth"), String::from("19890321"));
    attributes.insert(String::from("Street"), String::from("Main Road 207"));
    attributes.insert(String::from("City"), String::from("New York"));
    attributes.insert(String::from("Bank"), String::from("qBank New York"));
    attributes.insert(
        String::from("Card Number"),
        String::from("4537-6696-0666-0146"),
    );
    attributes.insert(String::from("Security Code"), String::from("063"));
    attributes.insert(String::from("Valid Until"), String::from("20251212"));
    debug!("Mock credential:\n{:#?}", attributes);

    let automation = CredentialOfferAutomation {
        connection_id,
        attributes,
    };

    automation.execute(agent).await
}
