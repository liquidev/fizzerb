use druid::{
    widget::{prelude::*, Label, LabelText},
    Affine, Data, Widget,
};

pub struct Button<T> {
    inner: Label<T>,
    inner_size: Size,
}

impl<T> Button<T>
where
    T: Data,
{
    pub fn new(inner: impl Into<LabelText<T>>) -> Self {
        Self::from_label(Label::new(inner).with_font(style::LABELLED_FONT))
    }

    pub fn from_label(label: Label<T>) -> Self {
        Self {
            inner: label,
            inner_size: Size::ZERO,
        }
    }
}

impl<T> Widget<T> for Button<T>
where
    T: Data,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::FocusChanged(_) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let horizontal_padding = env.get(style::LABELLED_PADDING);
        let height = env.get(style::LABELLED_HEIGHT);
        let bc = bc
            .shrink((horizontal_padding, 0.0))
            .shrink_max_height_to(height)
            .loosen();
        self.inner_size = self.inner.layout(ctx, &bc, data, env);
        bc.constrain((self.inner_size.width + horizontal_padding, height))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let color = if ctx.is_active() {
            env.get(style::ACTIVE_COLOR)
        } else if ctx.is_hot() {
            env.get(style::HOT_COLOR)
        } else {
            env.get(style::INACTIVE_COLOR)
        };
        ctx.fill(size.to_rounded_rect(size.height), &color);

        let inner_offset = size.to_vec2() / 2.0 - (self.inner_size / 2.0).to_vec2();
        let inner_offset = inner_offset.round();
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(inner_offset));
            self.inner.paint(ctx, data, env);
        });
    }
}

pub mod style {
    use druid::{Color, Env, FontDescriptor, Key};

    use crate::style::color;

    pub const LABELLED_HEIGHT: Key<f64> = style_key!("button.labelled.height");
    pub const LABELLED_PADDING: Key<f64> = style_key!("button.labelled.padding");
    pub const LABELLED_FONT: Key<FontDescriptor> = style_key!("button.labelled.font");

    pub const INACTIVE_COLOR: Key<Color> = style_key!("button.inactive.color");
    pub const HOT_COLOR: Key<Color> = style_key!("button.hot.color");
    pub const ACTIVE_COLOR: Key<Color> = style_key!("button.active.color");

    pub fn configure_env(env: &mut Env) {
        env.set(LABELLED_HEIGHT, 36.0);
        env.set(LABELLED_PADDING, 24.0);
        env.set(LABELLED_FONT, env.get(crate::style::TEXT));

        env.set(INACTIVE_COLOR, color(0xE2E5E9));
        env.set(HOT_COLOR, color(0xCDD3DA));
        env.set(ACTIVE_COLOR, color(0xA2AEBB));
    }
}
