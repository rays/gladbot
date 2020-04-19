extern crate discord;

use discord::model::Event;
use discord::Discord;
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;

#[derive(Debug)]

struct Character {
    nationality: String,
    style: String,
    hp: i8,
    ac: i8,
    strength: i8,
    agility: i8,
    stamina: i8,
    personality: i8,
    inteligence: i8,
    luck: i8,
    notes: String,
}

fn get_quote() -> String {
    let quotes = [
        "I Will Have My Vengeance",
        "Death smiles at us all. All a man can do is smile back",
        "Only a Famous Death Will Do",
        "Win the Crowd and Win the Freedom",
        "Honor Rome",
        "I Do What I Want to Do",
        "I Kill Because I'm Required",
        "Honor Maximus",
        "At my signal, unleash hell",
        "The frost, it sometimes makes the blade stick",
        "Nothing happens to anyone that he is not fitted by nature to bear",
        "What we do in life... echoes in eternity",
        "Fear and wonder, a powerful combination",
        "When a man sees his end... he wants to know there was some purpose to his life",
        "I am required to kill, so I kill. That is enough",
    ];

    let quote = quotes.choose(&mut rand::thread_rng()).unwrap().to_string();
    return quote;
}

fn roller(num_die: i8, die_type: i8) -> i8 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    let mut i = 0;

    while i < num_die {
        let roll = rng.gen_range(1, die_type+1);
        result = result + roll;
        i = i + 1;
    }
    return result;
}

fn calc_modifier(stat: i8) -> i8 {
    match stat {
        1..=3 => return -3,
        4..=5 => return -2,
        6..=8 => return -1,
        13..=15 => return 1,
        16..=17 => return 2,
        18 => return 3,
        _ => return 0,
    }
}

fn calc_hp(stamina: i8, luck: i8, nationality: String) -> i8 {
    let hp: i8;
    match nationality.as_str() {
        "Macedonian" => hp = roller(2, 4) + calc_modifier(stamina) + calc_modifier(luck),
        _ => hp = roller(2, 4) + calc_modifier(stamina),
    };

    return hp;
}

fn calc_ac(agility: i8, style: &String) -> i8 {
    let agility_mod = calc_modifier(agility);
    let mut ac = 10;

    let manica = 1;
    let shield = 1;
    let leather = 2;
    let large_shield = 2;
    let hide = 3;
    let scale = 4;
    let breastplate = 3;

    match style.as_str() {
        "Bestiarius" | "Dimachaerus" => ac = ac + leather + agility_mod,
        "Velites" | "Hoplomachus" | "Eques" => ac = ac + shield + agility_mod,
        "Thrancian" => ac = ac + manica + shield + agility_mod,
        "Retiarius" => ac = ac + manica + agility_mod,
        "Murmillo" => ac = ac + large_shield + manica + agility_mod,
        "Provacator" => ac = ac + breastplate + large_shield + agility_mod,
        "Scissor" => ac = ac + hide + agility_mod,
        "Samnite" => ac = ac + large_shield + scale + agility_mod,
        "Cataphractarius" => ac = ac + scale + agility_mod,
        _ => ac = ac + agility_mod,
    };

    return ac;
}

fn load_notes(style: &String) -> String {
    let notes: String;

    match style.as_str() {
        "Andabatae" => notes = "Blinded with Short sword and no armor. -4 penalty to attack rolls, move only at half speed, +2 for opponents to hit.".to_string(),
        "Fugitivus" => notes = "Roll 1d4 modified by luck: <1 Unarmed, 1 Club, 2 Dagger, 3 Short Sword, 4 Hand Axe, 5 Spear, 6 Warhammer, 7 Long Sword".to_string(),
        "Pugilatus" => notes = "Cestus (2)".to_string(),
        "Bestiarius" => notes = "Hand axe, spear, leather armor".to_string(),
        "Velites" => notes = "Two javelins, shield".to_string(),
        "Thracian" => notes = "Manica, shield, Roll 1d3: 1 Dagger, 2 Sica, 3 Short sword".to_string(),
        "Hoplomachus" => notes = "Spear, short sword, shield, helmet".to_string(),
        "Retiarius" => notes = "Trident, net, dagger, manica".to_string(),
        "Murmillo" => notes = "Short sword, manica, large shield, helmet".to_string(),
        "Dimachaerus" => notes = "Two long swords, leather armor, helmet".to_string(),
        "Provacator" => notes = "Short sword, breastplate, helmet, large shield".to_string(),
        "Laquearius" => notes = "Dagger, lasso/whip/grappling hook, manica".to_string(),   
        "Scissor" => notes = "Short sword, hide armor, scissor".to_string(),
        "Samnite" => notes = "Short sword, large shield, scale mail".to_string(),
        "Cataphractarius" => notes = "Polearm and scale mail".to_string(),
        "Rudiarius" => notes = "2d100 GP starting funds for initial weapons/armor".to_string(),
        "Sagittarius" => notes = "short bow, 20 arrows, horse, dagger".to_string(),
        "Eques" => notes = "Javelin, long sword, shield, helmet, horse".to_string(),
        "Essedarius" => notes = "Spear, helmet, chariot".to_string(),
        _ => notes = "".to_string(),
    };

    return notes;
}

fn find_style(luck: i8) -> String {
    let luck_modifier = calc_modifier(luck);

    let styles = [
        "Andabatae",
        "Fugitivus",
        "Pugilatus",
        "Bestiarius",
        "Velites",
        "Thracian",
        "Hoplomachus",
        "Retiarius",
        "Murmillo",
        "Dimachaerus",
        "Provacator",
        "Laquearius",
        "Scissor",
        "Samnite",
        "Cataphractarius",
        "Rudiarius",
        "Sagittarius",
        "Eques",
        "Essedarius",
    ];

    let mut roll = roller(2, 10) + luck_modifier - 1;
    if roll < 0 {
        roll = 0;
    }
    let style = styles.get(roll as usize);

    return style.unwrap().to_string();
}

fn gen_character() -> Character {
    let nationalities = [
        "Roman",
        "Carthaginian",
        "Egyptian",
        "Gaul",
        "Germanian",
        "Greek",
        "Illyrian",
        "Judaean",
        "Lycian",
        "Macedonian",
        "Numidian",
        "Parthian",
        "Syrian",
        "Thracian",
    ];

    let strength = roller(3, 6);
    let agility = roller(3, 6);
    let stamina = roller(3, 6);
    let personality = roller(3, 6);
    let inteligence = roller(3, 6);
    let luck = roller(3, 6);

    let nationality = nationalities.choose(&mut rand::thread_rng()).unwrap();
    let style = find_style(luck);
    let hp = calc_hp(stamina, luck, nationality.to_string());
    let ac = calc_ac(agility, &style);
    let notes = load_notes(&style);

    let character = Character {
        nationality: nationality.to_string(),
        style: style,
        hp: hp,
        ac: ac,
        strength: strength,
        agility: agility,
        stamina: stamina,
        personality: personality,
        inteligence: inteligence,
        luck: luck,
        notes: notes,
    };
    return character;
}

fn main() {
    let discord =
        Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN"))
            .expect("Login failed");

    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                let message_size = message.content.chars().count();
                if message.content == "!glad" {
                    println!("{} asked me to create a new bot", message.author.name);
                    let glad = gen_character();
                    let msg = format! {"A new gladiator has entered the arena\r\n
                    Nationality: {}; Style: {}; HP: {}; AC: {};
                    Str: {} ({}); Agi: {} ({}); Sta: {} ({}); Per: {} ({}); Int: {} ({}); Luc: {} ({});
                    Notes: {}", glad.nationality, glad.style, glad.hp, glad.ac,
                    glad.strength, calc_modifier(glad.strength),
                    glad.agility, calc_modifier(glad.agility),
                    glad.stamina, calc_modifier(glad.stamina),
                    glad.personality, calc_modifier(glad.personality),
                    glad.inteligence, calc_modifier(glad.inteligence),
                    glad.luck, calc_modifier(glad.luck),
                    glad.notes};
                    let _ = discord.send_message(message.channel_id, &msg, "", false);
                } else if message.content.to_uppercase() == message.content
                    && message_size >= 10
                    && message.author.name != "gladbot"
                {
                    println!("{} shouted: {}", message.author.name, message.content);
                    let quote = get_quote().to_uppercase();
                    let _ = discord.send_message(message.channel_id, &quote, "", false);
                } else if message.content == "!quit" {
                    println!("{} asked to me to quit", message.author.name);
                    let _ = discord.send_message(
                        message.channel_id,
                        "Gladbot is going away, bye!",
                        "",
                        false,
                    );
                    println!("Quitting.");
                    break;
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
