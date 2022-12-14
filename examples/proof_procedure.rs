use nextid_sdk::{
    proof_service::{Action, Endpoint, Platform, ProofProcedure},
    types::Result,
    util::{base64_encode, crypto::Secp256k1KeyPair, hex_encode},
};

fn gets() -> Option<String> {
    let mut temp = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    if temp.trim().len() == 0 {
        None
    } else {
        Some(temp.trim().to_string())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Which twitter username you want to bind?");
    let twitter_username = gets().expect("Twitter username must be provided.");

    println!("OK. Tell me your avatar public key:");
    let avatar: Secp256k1KeyPair;
    match gets() {
        None => {
            println!("Seems like you don't have an avatar yet. Let me generate one for you:");
            let mut rng = rand::rngs::OsRng;
            avatar = Secp256k1KeyPair::generate(&mut rng);
            println!(
                "Secret key: 0x{}",
                hex_encode(&avatar.sk.as_ref().unwrap().serialize())
            );
        }
        Some(pubkey_hex) => {
            avatar = Secp256k1KeyPair::from_pk_hex(&pubkey_hex)?;
        }
    }
    let avatar_pubkey = format!("0x{}", hex_encode(&avatar.pk.serialize_compressed()))
        .trim()
        .to_string();
    println!("Public key: {}", avatar_pubkey);

    // Procedure begins.
    let mut procedure = ProofProcedure::new(
        Endpoint::Staging,
        Action::Create,
        avatar,
        Platform::Twitter,
        &twitter_username,
    );
    procedure.get_payload().await?;

    let post_contents = procedure.post_content.clone().unwrap();
    let mut post_content = post_contents.get("default").unwrap().clone();

    if procedure.avatar.has_sk() {
        let personal_sign = procedure
            .avatar
            .personal_sign(procedure.sign_payload.as_ref().unwrap())?;
        post_content = post_content.replace("%SIG_BASE64%", &base64_encode(&personal_sign));
        println!("Let user post the following content as a public tweet:\n");
        println!(
            "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
        );
        print!("{}", post_content);
        println!(
            "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n"
        );
    } else {
        println!("Ask user to sign this using their avatar secret key using web3.eth.personal.sign() method:\n\n{}\n\n", procedure.sign_payload.as_ref().unwrap());
        println!("And then let user post the following content as a public tweet:\n");
        println!(
            "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
        );
        print!("{}", post_content);
        println!(
            "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n"
        );
        println!(
            "Remember to replace '%SIG_BASE64%' to base64-ed signature just created by user.\n"
        );
    }

    println!("Done? Good, tell me the tweet ID user just posted. (VERY_LONG_DIGITS in https://twitter.com/my_twitter_username/status/VERY_LONG_DIGITS)\n");
    let tweet_status_id = gets().expect("Tweet ID must be provided.");
    procedure.submit(tweet_status_id, None, None).await?;

    println!("Done.");

    Ok(())
}
