use hlua::Lua;
use std::fs::File;
use std::path::Path;

use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
struct Player {
    id: i32,
    hp: i32,
    zeny: i32,
}

fn custom_print(text: String) {
    println!("SCRIPT SAID: {:?}", text);
}

fn create_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS player (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            hp    INTEGER NOT NULL,
            zeny  INTEGER NULL
        )",
        NO_PARAMS,
    )
    .unwrap();
}

fn create_player(conn: &Connection, player: &Player) {
    conn.execute(
        "INSERT INTO
            player (hp, zeny)
        VALUES
            (?1, ?2)",
        &[player.hp, player.zeny],
    )
    .unwrap();
}

fn setup_db(conn: &Connection) {
    create_db(conn);

    let player = Player {
        id: 0,
        hp: 1000,
        zeny: 1,
    };
    create_player(&conn, &player);
}

fn find_player_by_id(conn: &Connection, id: &i32) -> Result<Player, rusqlite::Error> {
    let mut stmt = conn
        .prepare(
            "SELECT * FROM player
         WHERE id=?",
        )
        .unwrap();

    let player = stmt
        .query_map(&[&id], |row| Player {
            id: row.get(0),
            hp: row.get(1),
            zeny: row.get(2),
        })?
        .last()
        .unwrap();

    player
}

fn set_player_hp(conn: &Connection, id: &i32, amount: &i32) {
    conn.execute(
        "UPDATE player
             SET hp=?1
             WHERE id=?2",
        &[&amount, &id],
    )
    .unwrap();
}

fn main() {
    let db: String = std::env::args().nth(1).unwrap();

    let script: String = std::env::args().nth(2).unwrap();

    let player_id: i32 = std::env::args().nth(3).unwrap().parse().unwrap();

    let conn = Connection::open(db).unwrap();

    setup_db(&conn);

    {
        let mut lua = Lua::new();

        let get_hp = || {
            let player = match find_player_by_id(&conn, &player_id) {
                Ok(p) => p,
                Err(e) => panic!("Error {:?}", e),
            };

            player.hp
        };

        let set_hp = |amount: i32| {
            let player = match find_player_by_id(&conn, &player_id) {
                Ok(p) => p,
                Err(e) => panic!("Error {:?}", e),
            };

            set_player_hp(&conn, &player.id, &(player.hp + amount))
        };

        lua.set("print", hlua::function1(custom_print));
        lua.set("hp", hlua::function0(get_hp));
        lua.set("add_hp", hlua::function1(set_hp));

        lua.execute_from_reader::<(), _>(File::open(&Path::new(&script)).unwrap())
            .unwrap();
    }
}
