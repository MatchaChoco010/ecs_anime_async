use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use legion::*;

use super::components::*;

pub fn render(ctx: &mut Context, world: &mut World) -> GameResult<()> {
    graphics::clear(ctx, Color::new(0.95, 0.95, 0.95, 1.0));

    let mut renderable_data = <(&Renderable, &Position)>::query()
        .iter(world)
        .collect::<Vec<_>>();
    renderable_data.sort_by(|(_, p0), (_, p1)| p0.z.partial_cmp(&p1.z).unwrap());

    let draw_param = DrawParam::default();

    for (renderable, pos) in renderable_data {
        match renderable {
            Renderable::Circle { radius, color } => {
                let mesh = &Mesh::new_circle(
                    ctx,
                    DrawMode::Fill(FillOptions::default()),
                    na::Point2::new(pos.x, pos.y),
                    *radius,
                    1.0,
                    *color,
                )?;
                graphics::draw(ctx, mesh, draw_param)?
            }
            Renderable::Rectangle {
                width,
                height,
                color,
            } => {
                let mesh = &Mesh::new_rectangle(
                    ctx,
                    DrawMode::Fill(FillOptions::default()),
                    Rect::new(pos.x - width / 2.0, pos.y - height / 2.0, *width, *height),
                    *color,
                )?;
                graphics::draw(ctx, mesh, draw_param)?
            }
        }
    }

    graphics::present(ctx)?;

    Ok(())
}
