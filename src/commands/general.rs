use serenity::utils::Colour;

command!(amq(_ctx, msg, args) {

    if let Err(why) = msg.channel_id.send_message(|m| {
              //  let room = args.single::<f64>().unwrap();

                let (room, password) = match (args.single::<String>(), args.single::<String>()){
                    (Ok(room), Ok(password)) => (room, password),
                    _ => ("Gelo".to_owned(), "the obvious".to_owned()),
                };

                m.embed(|e| {
                    e.title("AMQ")
                        .url("https://animemusicquiz.com/")
                        .color(Colour::new(0xdc6676))
                        .image("https://i.imgur.com/ZYadmWW.png")
                        .field("Room", room, true)
                        .field("Password", password, true)
                })
            }) {
                println!("Error sending message: {:?}", why);
            }
});
