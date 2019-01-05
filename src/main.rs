use rlua::{Lua, UserData, UserDataMethods};
use std::fs;

#[derive(Debug)]
struct Player {
    id: String,
    hp: u8,
    zeny: u32,
}

impl UserData for Player {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("hp", |_, player, _: ()| Ok(player.hp));
        methods.add_method("zeny", |_, player, _: ()| Ok(player.zeny));

        methods.add_method_mut("add_zeny", |_, player, amount: u32| {
            player.zeny += amount;
            Ok(())
        });
    }
}

fn main() {
    let script = fs::read_to_string("foo.lua").expect("Something went wrong reading foo.lua file");

    let lua = Lua::new();

    let globals = lua.globals();

    let player = Player {
        id: String::from("123"),
        hp: 1,
        zeny: 1,
    };

    let console_print = lua.create_function(|_, text: (String)| {
        println!("FROM LUA: {:?}", text);
        Ok(())
    });

    globals.set("player", player).unwrap();

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

    globals.set("print", console_print.unwrap()).unwrap();

    lua.eval::<_, bool>(&script, None).unwrap();
}
