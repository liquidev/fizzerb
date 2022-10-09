use druid::{
    widget::{Controller, ControllerHost},
    Env, Event, EventCtx, KbKey, KeyEvent, Selector, Target, Widget,
};

macro_rules! command {
    ($name:tt) => {
        Selector::new(concat!("net.liquidev.fizzerb.", $name))
    };
}

pub const DELETE: Selector = command!("delete");

pub struct Commander;

impl Commander {
    fn consume_key(ctx: &mut EventCtx, keyboard: &KeyEvent) {
        let selector = match keyboard.key {
            KbKey::Delete => DELETE,
            _ => return,
        };
        ctx.submit_command(selector.with(()).to(Target::Widget(ctx.widget_id())));
        ctx.set_handled();
    }
}

impl<T, W> Controller<T, W> for Commander
where
    W: Widget<T>,
{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
            }
            Event::KeyDown(keyboard) => {
                if !keyboard.repeat {
                    Self::consume_key(ctx, keyboard);
                }
            }
            _ => (),
        }
        if !ctx.is_handled() {
            child.event(ctx, event, data, env);
        }
    }
}

/// Returns a widget that sends out commands to children.
pub fn commander<W>(child: W) -> ControllerHost<W, Commander> {
    ControllerHost::new(child, Commander)
}
