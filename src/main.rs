use std::time::Duration;

use anyhow::Result;
use futures::{join, select, FutureExt};
use ggez::event::KeyCode;
use ggez::graphics::Color;

mod app;
use app::components::*;
use app::*;

fn load_entities() {
    // Player HP
    app::push((
        Renderable::Rectangle {
            width: 150.0,
            height: 140.0,
            color: Color::from_rgba(50, 50, 50, 255),
        },
        Position {
            x: 150.0,
            y: 80.0,
            z: 10.0,
        },
    ));
    app::push((
        Renderable::Text {
            text: "Player".to_string(),
            color: Color::from_rgba(255, 255, 255, 255),
        },
        Position {
            x: 80.0,
            y: 30.0,
            z: 11.0,
        },
    ));
    app::push((
        Renderable::Text {
            text: "HP: 300/300".to_string(),
            color: Color::from_rgba(120, 255, 255, 255),
        },
        Position {
            x: 100.0,
            y: 60.0,
            z: 11.0,
        },
        PlayerHpText,
    ));
    app::push((
        Name {
            name: "player-hp-bar-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 120.0,
            height: 15.0,
            color: Color::from_rgba(120, 255, 255, 255),
        },
        Position {
            x: 150.0,
            y: 100.0,
            z: 11.0,
        },
        PlayerHpBar,
    ));

    // Enemy
    app::push((
        Name {
            name: "enemy".to_string(),
        },
        Renderable::Circle {
            radius: 60.0,
            color: Color::from_rgba(255, 128, 0, 0),
        },
        Position {
            x: 400.0,
            y: 300.0,
            z: 0.0,
        },
    ));
    app::push((
        Name {
            name: "enemy-hp-bar-window".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 30.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 400.0,
            y: 200.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "enemy-hp-bar-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 120.0,
            height: 10.0,
            color: Color::from_rgba(255, 255, 120, 0),
        },
        Position {
            x: 400.0,
            y: 200.0,
            z: 11.0,
        },
        EnemyHpBar,
    ));

    // Menu
    app::push((
        Name {
            name: "menu-window".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 140.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 250.0,
            y: 470.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "attack-select-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 25.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 420.0,
            z: 11.0,
        },
    ));
    app::push((
        Name {
            name: "attack-text".to_string(),
        },
        Renderable::Text {
            text: "Attack".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 200.0,
            y: 415.0,
            z: 12.0,
        },
    ));
    app::push((
        Name {
            name: "skill-select-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 25.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 450.0,
            z: 11.0,
        },
    ));
    app::push((
        Name {
            name: "skill-text".to_string(),
        },
        Renderable::Text {
            text: "Skill".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 200.0,
            y: 445.0,
            z: 12.0,
        },
    ));
    app::push((
        Name {
            name: "item-select-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 25.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 480.0,
            z: 11.0,
        },
    ));
    app::push((
        Name {
            name: "item-text".to_string(),
        },
        Renderable::Text {
            text: "Item".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 200.0,
            y: 475.0,
            z: 12.0,
        },
    ));
    app::push((
        Name {
            name: "escape-select-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 25.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 510.0,
            z: 11.0,
        },
    ));
    app::push((
        Name {
            name: "escape-text".to_string(),
        },
        Renderable::Text {
            text: "Escape".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 200.0,
            y: 505.0,
            z: 12.0,
        },
    ));

    // sub menu
    app::push((
        Name {
            name: "submenu-description-window".to_string(),
        },
        Renderable::Rectangle {
            width: 350.0,
            height: 150.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 510.0,
            y: 470.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "submenu-description-text".to_string(),
        },
        Renderable::Text {
            text: "Skill A description".to_string(),
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 360.0,
            y: 420.0,
            z: 11.0,
        },
        SubMenuDescriptionText,
    ));
    app::push((
        Name {
            name: "submenu-item-0-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 30.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 250.0,
            y: 410.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "submenu-item-0-highlight".to_string(),
        },
        Renderable::Rectangle {
            width: 140.0,
            height: 20.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 410.0,
            z: 11.0,
        },
        SubMenuHighlight { index: 0 },
    ));
    app::push((
        Name {
            name: "submenu-item-0-text".to_string(),
        },
        Renderable::Text {
            text: "Skill A".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 190.0,
            y: 400.0,
            z: 12.0,
        },
        SubMenuText { index: 0 },
    ));
    app::push((
        Name {
            name: "submenu-item-1-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 30.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 250.0,
            y: 450.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "submenu-item-1-highlight".to_string(),
        },
        Renderable::Rectangle {
            width: 140.0,
            height: 20.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 450.0,
            z: 11.0,
        },
        SubMenuHighlight { index: 1 },
    ));
    app::push((
        Name {
            name: "submenu-item-1-text".to_string(),
        },
        Renderable::Text {
            text: "Skill B".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 190.0,
            y: 440.0,
            z: 12.0,
        },
        SubMenuText { index: 1 },
    ));
    app::push((
        Name {
            name: "submenu-item-2-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 30.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 250.0,
            y: 490.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "submenu-item-2-highlight".to_string(),
        },
        Renderable::Rectangle {
            width: 140.0,
            height: 20.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 490.0,
            z: 11.0,
        },
        SubMenuHighlight { index: 2 },
    ));
    app::push((
        Name {
            name: "submenu-item-2-text".to_string(),
        },
        Renderable::Text {
            text: "Skill C".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 190.0,
            y: 480.0,
            z: 12.0,
        },
        SubMenuText { index: 2 },
    ));
    app::push((
        Name {
            name: "submenu-item-3-rect".to_string(),
        },
        Renderable::Rectangle {
            width: 150.0,
            height: 30.0,
            color: Color::from_rgba(50, 50, 50, 0),
        },
        Position {
            x: 250.0,
            y: 530.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "submenu-item-3-highlight".to_string(),
        },
        Renderable::Rectangle {
            width: 140.0,
            height: 20.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 530.0,
            z: 11.0,
        },
        SubMenuHighlight { index: 3 },
    ));
    app::push((
        Name {
            name: "submenu-item-3-text".to_string(),
        },
        Renderable::Text {
            text: "Skill D".to_string(),
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 190.0,
            y: 520.0,
            z: 12.0,
        },
        SubMenuText { index: 3 },
    ));
}

fn load_animations() -> Result<()> {
    app::load_animation("menu-fade-in", "./menu-fade-in.json")?;
    app::load_animation("menu-fade-out", "./menu-fade-out.json")?;
    app::load_animation("menu-attack-focus-in", "./menu-attack-focus-in.json")?;
    app::load_animation("menu-attack-focus-out", "./menu-attack-focus-out.json")?;
    app::load_animation("menu-skill-focus-in", "./menu-skill-focus-in.json")?;
    app::load_animation("menu-skill-focus-out", "./menu-skill-focus-out.json")?;
    app::load_animation("menu-item-focus-in", "./menu-item-focus-in.json")?;
    app::load_animation("menu-item-focus-out", "./menu-item-focus-out.json")?;
    app::load_animation("menu-escape-focus-in", "./menu-escape-focus-in.json")?;
    app::load_animation("menu-escape-focus-out", "./menu-escape-focus-out.json")?;
    app::load_animation("enemy-attack", "./enemy-attack.json")?;
    app::load_animation("enemy-damage", "./enemy-damage.json")?;
    app::load_animation("enemy-down", "./enemy-down.json")?;
    app::load_animation("encounter-enemy", "./encounter-enemy.json")?;
    app::load_animation("submenu-fade-in", "./submenu-fade-in.json")?;
    app::load_animation("submenu-fade-out", "./submenu-fade-out.json")?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum MenuResult {
    Attack,
    Skill,
    Item,
    Escape,
}
async fn menu_select() -> MenuResult {
    app::play_animation("menu-fade-in").await;
    let mut result = MenuResult::Attack;
    loop {
        select! {
            _ = app::key_press(KeyCode::Z).fuse() => {
                match result {
                    MenuResult::Attack => {
                        app::play_animation("menu-attack-focus-out").await;
                        app::play_animation("menu-fade-out").await;
                    }
                    MenuResult::Skill => {
                        app::play_animation("menu-skill-focus-out").await;
                        app::play_animation("menu-fade-out").await;
                    }
                    MenuResult::Item => {
                        app::play_animation("menu-item-focus-out").await;
                        app::play_animation("menu-fade-out").await;
                    }
                    MenuResult::Escape => {
                        app::play_animation("menu-escape-focus-out").await;
                        app::play_animation("menu-fade-out").await;
                    }
                }
                return result;
            }
            _ = app::key_press(KeyCode::Up).fuse() => {
                match result {
                    MenuResult::Attack => {
                        join!(
                            app::play_animation("menu-attack-focus-out"),
                            app::play_animation("menu-escape-focus-in")
                        );
                        result = MenuResult::Escape;
                    }
                    MenuResult::Skill => {
                        join!(
                            app::play_animation("menu-skill-focus-out"),
                            app::play_animation("menu-attack-focus-in")
                        );
                        result = MenuResult::Attack;
                    }
                    MenuResult::Item => {
                        join!(
                            app::play_animation("menu-item-focus-out"),
                            app::play_animation("menu-skill-focus-in")
                        );
                        result = MenuResult::Skill;
                    }
                    MenuResult::Escape => {
                        join!(
                            app::play_animation("menu-escape-focus-out"),
                            app::play_animation("menu-item-focus-in")
                        );
                        result = MenuResult::Item;
                    }
                }
            }
            _ = app::key_press(KeyCode::Down).fuse() => {
                match result {
                    MenuResult::Attack => {
                        join!(
                            app::play_animation("menu-attack-focus-out"),
                            app::play_animation("menu-skill-focus-in")
                        );
                        result = MenuResult::Skill;
                    }
                    MenuResult::Skill => {
                        join!(
                            app::play_animation("menu-skill-focus-out"),
                            app::play_animation("menu-item-focus-in")
                        );
                        result = MenuResult::Item;
                    }
                    MenuResult::Item => {
                        join!(
                            app::play_animation("menu-item-focus-out"),
                            app::play_animation("menu-escape-focus-in")
                        );
                        result = MenuResult::Escape;
                    }
                    MenuResult::Escape => {
                        join!(
                            app::play_animation("menu-escape-focus-out"),
                            app::play_animation("menu-attack-focus-in")
                        );
                        result = MenuResult::Attack;
                    }
                }
            }
        }
    }
}

enum SubMenuResult {
    Select(usize),
    Cancel,
}
async fn submenu<S: ToString>(items: &[S], descriptions: &[S]) -> SubMenuResult {
    let mut index = 0;
    let len = items.len();

    let mut view_top = 0;

    let mut ret = SubMenuResult::Cancel;

    app::change_submenu_item_text(&items[view_top..(view_top + 4.min(len))]);
    app::change_submenu_description_text(descriptions[index].to_string());

    app::play_animation("submenu-fade-in").await;

    loop {
        select! {
            _ = app::key_press(KeyCode::Down).fuse() => {
                let prev_index = index;
                index += 1;
                if index >= len {
                    index = 0;
                }

                let prev_view_top = view_top;
                view_top = index.saturating_sub(3);

                join!(
                    app::fadeout_submenu_highlight_item(prev_index - prev_view_top),
                    app::fadeout_submenu_description_text(),
                );
                app::change_submenu_item_text(&items[view_top..(view_top + 4.min(len))]);
                app::change_submenu_description_text(descriptions[index].to_string());
                join!(
                    app::fadein_submenu_highlight_item(index - view_top),
                    app::fadein_submenu_description_text(),
                );
            }
            _ = app::key_press(KeyCode::Up).fuse() => {
                let prev_index = index;
                if index <= 0 {
                    index = len - 1;
                } else {
                    index -= 1;
                }

                let prev_view_top = view_top;
                if view_top > index {
                    view_top = index
                }
                if index == len - 1 {
                    view_top = index.saturating_sub(3);
                }

                join!(
                    app::fadeout_submenu_highlight_item(prev_index - prev_view_top),
                    app::fadeout_submenu_description_text(),
                );
                app::change_submenu_item_text(&items[view_top..(view_top + 4.min(len))]);
                app::change_submenu_description_text(descriptions[index].to_string());
                join!(
                    app::fadein_submenu_highlight_item(index - view_top),
                    app::fadein_submenu_description_text(),
                );
            }
            _ = app::key_press(KeyCode::X).fuse() => {
                break;
            }
            _ = app::key_press(KeyCode::Z).fuse() => {
                ret = SubMenuResult::Select(index);
                break;
            }
        }
    }

    app::play_animation("submenu-fade-out").await;

    ret
}

enum SkillMenuResult {
    Cancel,
    Select(PlayerTurnResult),
}
async fn skill_menu() -> SkillMenuResult {
    let skills = [
        "Skill A", "Skill B", "Skill C", "Skill D", "Skill E", "Skill F",
    ];
    let descriptions = [
        "Deals 150 damage to enemies.",
        "Deals 250 damage to enemies.",
        "Deals 300 damage to enemies.",
        "Deals 400 damage to enemies.",
        "Deals 500 damage to enemies.",
        "Deals 750 damage to enemies.",
    ];
    let damages = [150, 250, 300, 400, 500, 750];
    match submenu(&skills, &descriptions).await {
        SubMenuResult::Cancel => SkillMenuResult::Cancel,
        SubMenuResult::Select(index) => {
            app::add_message(format!("Use {}!", skills[index]));
            runtime::next_frame().await;

            SkillMenuResult::Select(PlayerTurnResult::EnemyDamage(damages[index]))
        }
    }
}

enum ItemMenuResult {
    Cancel,
    Select(PlayerTurnResult),
}
async fn item_menu(items: &mut Vec<Item>) -> ItemMenuResult {
    if items.is_empty() {
        app::add_message("You do not possess the item!");
        return ItemMenuResult::Cancel;
    }

    let names = items.iter().map(|i| i.name.clone()).collect::<Vec<_>>();
    let descriptions = items
        .iter()
        .map(|i| i.description.clone())
        .collect::<Vec<_>>();
    let effects = items.iter().map(|i| i.effect.clone()).collect::<Vec<_>>();
    match submenu(&names, &descriptions).await {
        SubMenuResult::Cancel => ItemMenuResult::Cancel,
        SubMenuResult::Select(index) => {
            let item = items.remove(index);

            app::add_message(format!("Use a {} item!", item.name));
            runtime::next_frame().await;

            ItemMenuResult::Select(effects[index].clone())
        }
    }
}

#[derive(Clone)]
enum PlayerTurnResult {
    EnemyDamage(i32),
    PlayerRecovery(i32),
    Escape,
}
async fn player_turn(items: &mut Vec<Item>) -> PlayerTurnResult {
    loop {
        let ret = menu_select().await;
        match ret {
            MenuResult::Attack => {
                app::add_message("Attack of the Player!");
                app::play_animation("enemy-damage").await;
                return PlayerTurnResult::EnemyDamage(100);
            }
            MenuResult::Skill => match skill_menu().await {
                SkillMenuResult::Cancel => (),
                SkillMenuResult::Select(effect) => return effect,
            },
            MenuResult::Item => match item_menu(items).await {
                ItemMenuResult::Cancel => (),
                ItemMenuResult::Select(effect) => return effect,
            },
            MenuResult::Escape => {
                app::add_message("You can't run from this fight!");
                runtime::next_frame().await;
                return PlayerTurnResult::Escape;
            }
        }
    }
}

enum EnemyTurnResult {
    PlayerDamage(i32),
}
async fn enemy_turn() -> EnemyTurnResult {
    app::add_message("Attack of the enemy!");
    app::play_animation("enemy-attack").await;
    EnemyTurnResult::PlayerDamage(35)
}

struct Item {
    name: String,
    description: String,
    effect: PlayerTurnResult,
}
fn create_items() -> Vec<Item> {
    vec![
        Item {
            name: "Potion 1".to_string(),
            description: "Heals the player 100".to_string(),
            effect: PlayerTurnResult::PlayerRecovery(100),
        },
        Item {
            name: "Potion 1".to_string(),
            description: "Heals the player 100".to_string(),
            effect: PlayerTurnResult::PlayerRecovery(100),
        },
        Item {
            name: "Potion 1".to_string(),
            description: "Heals the player 100".to_string(),
            effect: PlayerTurnResult::PlayerRecovery(100),
        },
        Item {
            name: "Potion 2".to_string(),
            description: "Heals the player 300".to_string(),
            effect: PlayerTurnResult::PlayerRecovery(300),
        },
        Item {
            name: "Fire Scroll".to_string(),
            description: "Deals 1000 damage to enemies.".to_string(),
            effect: PlayerTurnResult::EnemyDamage(1000),
        },
    ]
}

fn main() -> Result<()> {
    let mut app = App::new("app", "Orito Itsuki")?;

    load_entities();
    load_animations()?;

    runtime::spawn(async {
        let player_max_hp = 300;
        let mut player_hp = 300;
        app::set_player_hp_bar(player_hp, player_max_hp);

        loop {
            app::key_press(KeyCode::Z).await;

            let enemy_max_hp = 1500;
            let mut enemy_hp = 1500;
            app::set_enemy_hp_bar(enemy_hp, enemy_max_hp);
            let mut items = create_items();

            app::add_message("Encounter the enemy!");
            app::play_animation("encounter-enemy").await;

            'battle: loop {
                match player_turn(&mut items).await {
                    PlayerTurnResult::EnemyDamage(damage) => {
                        enemy_hp -= damage;
                        enemy_hp = enemy_hp.max(0);
                        app::set_enemy_hp_bar(enemy_hp, enemy_max_hp);
                        app::add_message(format!("{} damage to enemies!", damage));
                        app::add_enemy_damage_effect(damage);

                        if enemy_hp <= 0 {
                            app::play_animation("enemy-down").await;
                            app::add_message("The enemy is down!");
                            break 'battle;
                        }
                    }
                    PlayerTurnResult::PlayerRecovery(recovery) => {
                        player_hp += recovery;
                        player_hp = player_hp.min(player_max_hp);
                        app::set_player_hp_bar(player_hp, player_max_hp);
                        app::add_message(format!("Recover Player {}!", recovery));
                        app::add_player_recovery_effect(recovery);
                    }
                    PlayerTurnResult::Escape => (),
                }

                runtime::delay(Duration::from_secs_f64(1.2)).await;

                match enemy_turn().await {
                    EnemyTurnResult::PlayerDamage(damage) => {
                        player_hp -= damage;
                        player_hp = player_hp.max(0);
                        app::set_player_hp_bar(player_hp, player_max_hp);
                        app::add_message(format!("{} damage to Player!", damage));
                        app::add_player_damage_effect(damage);
                    }
                }

                if player_hp <= 0 {
                    runtime::next_frame().await;
                    app::add_message("Game Over...");
                    app::key_press(KeyCode::Z).await;
                    player_hp = 300;
                    app::set_player_hp_bar(player_hp, player_max_hp);
                    break 'battle;
                }
            }
        }
    });

    app.run()
}
