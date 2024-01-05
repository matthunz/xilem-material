use std::time::{Duration, Instant};
use xilem::{
    vello::{
        kurbo::{Affine, Circle, Point},
        peniko::{Brush, Color},
    },
    view::{Id, View},
    widget::{ChangeFlags, Event, Widget},
    MessageResult,
};

pub struct Ripple {
    start: Instant,
    duration: Duration,
}

impl Ripple {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            duration: Duration::from_millis(300),
        }
    }
}

impl<T, A> View<T, A> for Ripple {
    type State = Instant;
    type Element = RippleWidget;

    fn build(&self, cx: &mut xilem::view::Cx) -> (Id, Self::State, Self::Element) {
        let (id, elem) = cx.with_new_id(|_cx| RippleWidget {
            start: None,
            duration: self.duration,
        });
        (id, Instant::now(), elem)
    }

    fn rebuild(
        &self,
        _cx: &mut xilem::view::Cx,
        _prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        _element: &mut Self::Element,
    ) -> xilem::widget::ChangeFlags {
        ChangeFlags::PAINT
    }

    fn message(
        &self,
        _id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        *state = *message.downcast().unwrap();
        MessageResult::RequestRebuild
    }
}

pub struct RippleWidget {
    start: Option<(Instant, Point)>,
    duration: Duration,
}

impl Default for RippleWidget {
    fn default() -> Self {
        Self {
            start: None,
            duration: Duration::from_millis(300),
        }
    }
}

impl Widget for RippleWidget {
    fn event(&mut self, cx: &mut xilem::widget::EventCx, event: &Event) {
        match event {
            Event::MouseDown(mouse_event) => {
                cx.request_paint();
                self.start = Some((Instant::now(), mouse_event.pos));
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _cx: &mut xilem::widget::LifeCycleCx,
        _event: &xilem::widget::LifeCycle,
    ) {
    }

    fn update(&mut self, _cx: &mut xilem::widget::UpdateCx) {}

    fn layout(
        &mut self,
        _cx: &mut xilem::widget::LayoutCx,
        bc: &xilem::widget::BoxConstraints,
    ) -> xilem::vello::kurbo::Size {
        bc.max()
    }

    fn accessibility(&mut self, _cx: &mut xilem::widget::AccessCx) {}

    fn paint(&mut self, cx: &mut xilem::widget::PaintCx, builder: &mut xilem::vello::SceneBuilder) {
        let max_radius = cx.size().max_side();

        if let Some((start, pos)) = self.start {
            let scale = Instant::now()
                .checked_duration_since(start)
                .unwrap_or_default()
                .as_secs_f64()
                / self.duration.as_secs_f64();
            let radius = max_radius.min(scale * max_radius);
            let opacity = if scale < 0.3 {
                scale
            } else {
                0.3f64.min(1. - scale)
            };

            builder.fill(
                xilem::vello::peniko::Fill::EvenOdd,
                Affine::default(),
                &Brush::Solid(Color::rgba(1., 1., 1., opacity)),
                None,
                &Circle::new((pos.x, pos.y), radius),
            );
        }
    }
}
