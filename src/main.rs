use rlua::{Lua, UserData, UserDataMethods};
use std::fs;

struct Player {
    id: String,
    hp: u8,
    zeny: u32,
}

// impl Player {
//     fn new(id: String, hp: u8, zeny: u32) -> Player {
//         Player {
//             id: id,
//             hp: hp,
//             zeny: zeny,
//         }
//     }
// }

impl UserData for Player {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("hp", |_, player, ()| Ok(player.hp));
    }
}

fn find_player_by_id(id: String, players: &Vec<Player>) -> Option<&Player> {
    players.into_iter().find(|&player| player.id == id)
}

fn main() {
    let script = fs::read_to_string("foo.lua").expect("Something went wrong reading foo.lua file");

    let lua = Lua::new();

    let globals = lua.globals();

    let check_equal =
        lua.create_function(|_, (list1, list2): (Vec<String>, Vec<String>)| Ok(list1 == list2));

    let console_print = lua.create_function(|_, text: (String)| {
        println!("FROM LUA: {:?}", text);
        Ok(())
    });

    let player_constructor = lua.create_function(|_, ()| {
        Ok(Player {
            id: String::from("foo"),
            hp: 10,
            zeny: 0,
        })
    });
    globals.set("player", player_constructor.unwrap());

    // let getPlayer = lua.create_function(move |_, id: (String)| {
    //     let Some(player) = find_player_by_id(id, &players);
    //     Ok(player)
    // });

    // let getPlayerHP = lua.create_function(move |_, id: (String)| {
    //     Ok(match find_player_by_id(id, &players) {
    //         Some(player) => player.hp,
    //         None => 0,
    //     })
    // });

    globals.set("check_equal", check_equal.unwrap());
    globals.set("print", console_print.unwrap());
    // globals.set("getPlayerHp", getPlayerHP.unwrap());
    lua.eval::<_, bool>(&script, None);
}
