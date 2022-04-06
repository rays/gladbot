use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use handlebars::Handlebars;
use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::Rng;
use std::env;

use rnglib::{Language, RNG};
use rusqlite::{params, Connection, Result};

use tokio;

#[group]
#[commands(glad, taunt, fight)]

struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[derive(Clone, Debug)]
struct Character {
    name: String,
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
    initiative: i8,
    weapon: Weapon,
}

#[derive(Clone, Debug)]
struct Weapon {
    name: String,
    damage_die: i8,
    is_melee: bool,
}

fn get_weapon(weapon_key: String) -> Weapon {
    let mut weapon_table = HashMap::new();
    println!("{}", weapon_key);

    weapon_table.insert(
        "Fists".to_string(),
        Weapon {
            name: "Fists".to_string(),
            damage_die: 3,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Sica".to_string(),
        Weapon {
            name: "Sica".to_string(),
            damage_die: 5,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Warhammer".to_string(),
        Weapon {
            name: "Warhammer".to_string(),
            damage_die: 8,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Short Sword".to_string(),
        Weapon {
            name: "Short Sword".to_string(),
            damage_die: 6,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Cestus".to_string(),
        Weapon {
            name: "Cestus".to_string(),
            damage_die: 3,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Hand Axe".to_string(),
        Weapon {
            name: "Hand Axe".to_string(),
            damage_die: 6,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Spear".to_string(),
        Weapon {
            name: "Spear".to_string(),
            damage_die: 8,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Javelin".to_string(),
        Weapon {
            name: "Javelin".to_string(),
            damage_die: 6,
            is_melee: false,
        },
    );
    weapon_table.insert(
        "Trident".to_string(),
        Weapon {
            name: "Trident".to_string(),
            damage_die: 7,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Long Sword".to_string(),
        Weapon {
            name: "Long Sword".to_string(),
            damage_die: 8,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Polearm".to_string(),
        Weapon {
            name: "Polearm".to_string(),
            damage_die: 10,
            is_melee: true,
        },
    );
    weapon_table.insert(
        "Shortbow".to_string(),
        Weapon {
            name: "Shortbow".to_string(),
            damage_die: 6,
            is_melee: false,
        },
    );
    weapon_table.insert(
        "Dagger".to_string(),
        Weapon {
            name: "Dagger".to_string(),
            damage_die: 4,
            is_melee: false,
        },
    );
    weapon_table[&weapon_key].clone()
}

fn get_quote() -> String {
    let quotes = [
        "Death smiles at us all. All a man can do is smile back",
        "Only a Famous Death Will Do",
        "Win the Crowd and Win the Freedom",
        "Honor Rome",
        "Honor Maximus",
        "At my signal, unleash hell",
    ];

    let quote = quotes.choose(&mut rand::thread_rng());
    (*quote.unwrap()).to_string()
}

fn get_hit_msg(weapon: String, attacker: String, opponent: String, damage: i8) -> String {
    let hit_msgs = [
        "{{ attacker }}'s {{ weapon }} strikes across {{ opponent }}'s chest, leaving a long, shallow gash [{{ damage }}]",
        "{{ opponent }} blocks {{ attacker}}'s {{ weapon }} and {{ attacker }} quickly lean into the block and smash the haft into whatever approximates for a mouth on {{ opponent }} [{{ damage }}]",
        "{{ attacker }}'s {{ weapon }} digs deep into the gut of {{ opponent }}, who groans painfully before expelling bloody spittle onto the ground [{{ damage }}]",
        "{{ attacker }}'s powerful swipe thier {{ weapon }} sends {{ opponent }}'s index finger flying [{{ damage }}]",
        "{{ attacker }} sidesteps {{ opponent }}'s swing, and counter with a strike to their leg [{{ damage }}]",
        "Vicious! {{ attacker }}'s {{ weapon }} scores a clean hit that shall be felt by {{ opponent }}'s ancestors! [{{ damage }}]",
        "{{ attacker }} bring their {{ weapon }} down upon {{ opponent }} for a devastating overhead strike [{{ damage }}]",
        "{{ attacker }} spins their {{ weapon }} with great skill and then bring it down upon {{ opponent }}'s shoulder with a shuddering crunch [{{ damage }}]",
        "{{ attacker }} thrusts their {{ weapon }} forward in a feint and then hits {{ opponent }} from behind as {{ attacker }} draws it back [{{ damage }}]",
        "{{ attacker }} fakes dropping their {{ weapon }} then catches it with their foot, and then kick it back at your {{ opponent }}'s face. And then {{ attacker }} catches it on the rebound. OOH YEAH! [{{ damage }}]",
        "{{ attacker }}'s brutal strike carves {{ opponent }} a third nostril [{{ damage }}]"
    ];

    let source = hit_msgs.choose(&mut rand::thread_rng());
    let source = (*source.unwrap()).to_string();

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("hit", source).unwrap();

    let mut data = HashMap::new();
    data.insert("weapon", weapon);
    data.insert("attacker", attacker);
    data.insert("opponent", opponent);
    data.insert("damage", damage.to_string());

    format!("{}", handlebars.render("hit", &data).unwrap())
}

fn roller(num_die: i8, die_type: i8) -> i8 {
    let mut rng = rand::thread_rng();
    let mut result = 0;
    let mut i = 0;

    while i < num_die {
        let roll = rng.gen_range(1..die_type + 1);
        result += roll;
        i += 1;
    }
    result
}

fn calc_modifier(stat: i8) -> i8 {
    match stat {
        1..=3 => -3,
        4..=5 => -2,
        6..=8 => -1,
        13..=15 => 1,
        16..=17 => 2,
        18 => 3,
        _ => 0,
    }
}

fn calc_hp(stamina: i8, luck: i8, nationality: String) -> i8 {
    let hp: i8;
    match nationality.as_str() {
        "Macedonian" => hp = roller(2, 4) + calc_modifier(stamina) + calc_modifier(luck),
        _ => hp = roller(2, 4) + calc_modifier(stamina),
    };

    hp
}

fn calc_ac(agility: i8, style: &str) -> i8 {
    let agility_mod = calc_modifier(agility);
    let mut ac = 10;

    let manica = 1;
    let shield = 1;
    let leather = 2;
    let large_shield = 2;
    let hide = 3;
    let scale = 4;
    let breastplate = 3;

    match style {
        "Bestiarius" | "Dimachaerus" => ac = ac + leather + agility_mod,
        "Velites" | "Hoplomachus" | "Eques" => ac = ac + shield + agility_mod,
        "Thracian" => ac = ac + manica + shield + agility_mod,
        "Retiarius" => ac = ac + manica + agility_mod,
        "Murmillo" => ac = ac + large_shield + manica + agility_mod,
        "Provacator" => ac = ac + breastplate + large_shield + agility_mod,
        "Scissor" => ac = ac + hide + agility_mod,
        "Samnite" => ac = ac + large_shield + scale + agility_mod,
        "Cataphractarius" => ac = ac + scale + agility_mod,
        _ => ac += agility_mod,
    };

    ac
}

fn load_weapon(style: &str) -> Weapon {
    let weapon: Weapon;

    match style {
        "Andabatae" => weapon = get_weapon("Short Sword".to_string()),
        "Fugitivus" => {
            let possible_weapons = [
                "Fists",
                "Club",
                "Dagger",
                "Short Sword",
                "Hand Axe",
                "Spear",
                "Warhammer",
                "Long Sword",
            ];
            let choice = possible_weapons.choose(&mut rand::thread_rng()).unwrap();
            weapon = get_weapon(choice.to_string());
        }
        "Pugilatus" => weapon = get_weapon("Cestus".to_string()),
        "Bestiarius" => weapon = get_weapon("Hand Axe".to_string()),
        "Velites" => weapon = get_weapon("Javelin".to_string()),
        "Thracian" => {
            let possible_weapons = ["Dagger", "Sica", "Short Sword"];
            let choice = possible_weapons.choose(&mut rand::thread_rng()).unwrap();
            weapon = get_weapon(choice.to_string());
        }
        "Hoplomachus" => weapon = get_weapon("Spear".to_string()),
        "Retiarius" => weapon = get_weapon("Trident".to_string()),
        "Murmillo" => weapon = get_weapon("Short Sword".to_string()),
        "Dimachaerus" => weapon = get_weapon("Long Sword".to_string()),
        "Provacator" => weapon = get_weapon("Short Sword".to_string()),
        "Laquearius" => weapon = get_weapon("Dagger".to_string()),
        "Scissor" => weapon = get_weapon("Short Sword".to_string()),
        "Samnite" => weapon = get_weapon("Short Sword".to_string()),
        "Cataphractarius" => weapon = get_weapon("Polearm".to_string()),
        // "Rudiarius" => notes = "2d100 GP starting funds for initial weapons/armor".to_string(),
        "Sagittarius" => weapon = get_weapon("Shortbow".to_string()),
        "Eques" => weapon = get_weapon("Javelin".to_string()),
        "Essedarius" => weapon = get_weapon("Spear".to_string()),
        _ => weapon = get_weapon("Fists".to_string()),
    };
    weapon
}

fn load_notes(style: &str) -> String {
    let notes: String;

    match style {
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

    notes
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

    (*style.unwrap()).to_string()
}

fn get_characters(num: i8) -> Result<Vec<Character>> {
    let db = Connection::open("/tmp/glad.db")?;

    let mut stmt = db.prepare("SELECT * FROM glads ORDER BY id DESC LIMIT ?1")?;
    let rows = stmt.query_map([num], |row| {
        Ok(Character {
            name: row.get(1).unwrap(),
            nationality: row.get(2).unwrap(),
            style: row.get(3).unwrap(),
            hp: row.get(4).unwrap(),
            ac: row.get(5).unwrap(),
            strength: row.get(6).unwrap(),
            agility: row.get(7).unwrap(),
            stamina: row.get(8).unwrap(),
            personality: row.get(9).unwrap(),
            inteligence: row.get(10).unwrap(),
            luck: row.get(11).unwrap(),
            notes: row.get(12).unwrap(),
            initiative: row.get(13).unwrap(),
            weapon: get_weapon(row.get(14).unwrap()),
        })
    })?;

    let mut characters = Vec::new();
    for row in rows {
        let character = row.unwrap();
        characters.push(character);
    }

    Ok(characters)
}

fn save_character(character: Character) -> Result<()> {
    let db = Connection::open("/tmp/glad.db")?;

    let _result = match db.execute_batch(
        "
    CREATE TABLE IF NOT EXISTS glads (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name STRING,
        nationality STRING,
        style STRING,
        hp INTEGER,
        ac INTEGER,
        strength INTEGER,
        agility INTEGER,
        stamina INTEGER,
        personality INTEGER,
        inteligence INTEGER,
        luck INTEGER,
        notes STRING,
        initiative INTEGER,
        weapon_key STRING
    );",
    ) {
        Ok(result) => result,
        Err(e) => {
            println!("error creating db: {}", e);
            return Err(e);
        }
    };

    let _result = match db.execute(
        "INSERT INTO glads VALUES (NULL, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            character.name,
            character.nationality,
            character.style,
            character.hp,
            character.ac,
            character.strength,
            character.agility,
            character.stamina,
            character.personality,
            character.inteligence,
            character.luck,
            character.notes,
            character.initiative,
            character.weapon.name
        ],
    ) {
        Ok(result) => result,
        Err(e) => {
            println!("error saving record: {}", e);
            return Err(e);
        }
    };

    Ok(())
}

fn gen_character() -> Character {
    let rng = RNG::new(&Language::Roman).unwrap();

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

    let name = rng.generate_name();
    let nationality = nationalities.choose(&mut rand::thread_rng()).unwrap();
    let style = find_style(luck);
    let hp = calc_hp(stamina, luck, (*nationality).to_string());
    let ac = calc_ac(agility, &style);
    let notes = load_notes(&style);
    let initiative = 0;
    let weapon = load_weapon(&style);

    Character {
        name: name.to_string(),
        nationality: (*nationality).to_string(),
        style,
        hp,
        ac,
        strength,
        agility,
        stamina,
        personality,
        inteligence,
        luck,
        notes,
        initiative,
        weapon,
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("GLADBOT_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn glad(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{} asked me to create a new gladiator!", msg.author.name);

    let glad = gen_character();
    let strength_mod = calc_modifier(glad.strength);
    let agility_mod = calc_modifier(glad.agility);
    let stamina_mod = calc_modifier(glad.stamina);
    let personality_mod = calc_modifier(glad.personality);
    let inteligence_mod = calc_modifier(glad.inteligence);
    let luck_mod = calc_modifier(glad.luck);

    save_character(glad.clone())?;

    let out = format! {"Gladiator {} has entered the arena!\n\nNationality: {}; Style: {}\nHP: {}; AC: {}\nStr: {} ({}); Agi: {} ({}); Sta: {} ({}); Per: {} ({}); Int: {} ({}); Luc: {} ({})\nNotes: {}",
    glad.name, glad.nationality,
    glad.style, glad.hp, glad.ac,
    glad.strength, strength_mod,
    glad.agility, agility_mod,
    glad.stamina, stamina_mod,
    glad.personality, personality_mod,
    glad.inteligence, inteligence_mod,
    glad.luck, luck_mod,
    glad.notes};
    msg.reply(ctx, &out).await?;
    println!("{}", &out);

    Ok(())
}

#[command]
async fn taunt(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{} asked me to taunt them!", msg.author.name);
    let quote = get_quote().to_uppercase();
    msg.reply(ctx, &quote).await?;

    Ok(())
}

#[command]
async fn fight(ctx: &Context, msg: &Message) -> CommandResult {
    let command = format!(
        "{} commands that two gladiators fight to the death!",
        msg.author.name
    );
    msg.reply(ctx.clone(), &command).await?;

    let quote = get_quote().to_uppercase();
    msg.reply(ctx.clone(), &quote).await?;
    let characters = get_characters(2).unwrap();

    // Roll for initiative
    let mut gladiators = Vec::new();
    for mut glad in characters.clone() {
        glad.initiative = roller(1, 20) + calc_modifier(glad.agility);
        gladiators.push(glad)
    }
    gladiators.sort_by_key(|d| d.initiative);
    gladiators.reverse();

    let mut glad1 = gladiators.pop().unwrap();
    let mut glad2 = gladiators.pop().unwrap();

    loop {
        println!("{}'s Current HP: {}", glad1.name, glad1.hp);
        println!("{}'s Current HP: {}", glad2.name, glad2.hp);

        // Gladiator 1
        let mut attack_modifier = calc_modifier(glad1.strength);
        let mut dmg_modifier = calc_modifier(glad1.strength);
        if !glad1.weapon.is_melee {
            attack_modifier = calc_modifier(glad1.agility);
            dmg_modifier = 0;
        }
        let to_hit = roller(1, 20) + attack_modifier;
        if to_hit >= glad2.ac {
            let dmg = roller(1, glad1.weapon.damage_die) + dmg_modifier;
            let status = get_hit_msg(
                glad1.weapon.name.clone(),
                glad1.name.clone(),
                glad2.name.clone(),
                dmg,
            );
            msg.reply(ctx.clone(), &status).await?;
            glad2.hp = glad2.hp - dmg;
            if glad2.hp <= 0 {
                let status = format!("{} has been defeated in mortal combat!", glad2.name);
                msg.reply(ctx.clone(), &status).await?;
                break;
            }
        } else {
            let status = format!("{} misses their attack", glad1.name);
            msg.reply(ctx.clone(), &status).await?;
        }

        // Gladiator 2
        let mut attack_modifier = calc_modifier(glad2.strength);
        let mut dmg_modifier = calc_modifier(glad2.strength);
        if !glad2.weapon.is_melee {
            attack_modifier = calc_modifier(glad2.agility);
            dmg_modifier = 0;
        }
        let to_hit = roller(1, 20) + attack_modifier;
        if to_hit >= glad1.ac {
            let dmg = roller(1, glad2.weapon.damage_die) + dmg_modifier;
            let status = get_hit_msg(
                glad2.weapon.name.clone(),
                glad2.name.clone(),
                glad1.name.clone(),
                dmg,
            );
            msg.reply(ctx.clone(), &status).await?;
            glad1.hp = glad1.hp - dmg;
            if glad1.hp <= 0 {
                let status = format!("{} has been defeated in mortal combat!", glad1.name);
                msg.reply(ctx.clone(), &status).await?;
                break;
            }
        } else {
            let status = format!("{} misses their attack", glad2.name);
            msg.reply(ctx.clone(), &status).await?;
        }
    }

    Ok(())
}
