use crate::RippleWidget;
use xilem::{
    vello::{
        kurbo::{Affine, Circle, Size},
        peniko::{BlendMode, Color, Stroke},
    },
    view::{Id, View, ViewMarker},
    widget::{ChangeFlags, Event, EventCx, Widget},
    MessageResult,
};

#[derive(Clone)]
pub struct RadioButton {
    outer_size: f64,
    inner_size: f64,
    target_size: f64,
}

impl Default for RadioButton {
    fn default() -> Self {
        Self {
            outer_size: 40.,
            inner_size: 20.,
            target_size: 48.,
        }
    }
}

impl ViewMarker for RadioButton {}

impl<T, A> View<T, A> for RadioButton {
    type State = ();

    type Element = RadioButtonWidget;

    fn build(&self, cx: &mut xilem::view::Cx) -> (Id, Self::State, Self::Element) {
        let (id, elem) = cx.with_new_id(|_cx| RadioButtonWidget::new(self.clone()));
        (id, (), elem)
    }

    fn rebuild(
        &self,
        _cx: &mut xilem::view::Cx,
        _prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        _element: &mut Self::Element,
    ) -> xilem::widget::ChangeFlags {
        ChangeFlags::all()
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        _message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Nop
    }
}

pub struct RadioButtonWidget {
    view: RadioButton,
    ripple: RippleWidget,
}

impl RadioButtonWidget {
    pub fn new(view: RadioButton) -> Self {
        Self {
            view,
            ripple: RippleWidget::default(),
        }
    }
}

impl Widget for RadioButtonWidget {
    fn event(&mut self, cx: &mut EventCx, event: &Event) {
        self.ripple.event(cx, event)
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
        _bc: &xilem::widget::BoxConstraints,
    ) -> xilem::vello::kurbo::Size {
        Size::new(self.view.target_size, self.view.target_size)
    }

    fn accessibility(&mut self, _cx: &mut xilem::widget::AccessCx) {}

    fn paint(&mut self, cx: &mut xilem::widget::PaintCx, builder: &mut xilem::vello::SceneBuilder) {
        let target_radius = self.view.target_size / 2.;
        let outer_radius = self.view.outer_size / 2.;

        builder.stroke(
            &Stroke::new(2.),
            Affine::default(),
            &Color::GREEN,
            None,
            &Circle::new((target_radius, target_radius), outer_radius),
        );

        let inner_radius = self.view.inner_size / 2.;
        builder.fill(
            xilem::vello::peniko::Fill::EvenOdd,
            Affine::default(),
            &Color::GREEN,
            None,
            &Circle::new((target_radius, target_radius), inner_radius),
        );

        builder.push_layer(
            BlendMode::new(
                xilem::vello::peniko::Mix::Clip,
                xilem::vello::peniko::Compose::Plus,
            ),
            1.,
            Affine::default(),
            &Circle::new((target_radius, target_radius), target_radius),
        );
        self.ripple.paint(cx, builder);
        builder.pop_layer();
    }
}
