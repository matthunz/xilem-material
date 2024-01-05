use crate::RippleWidget;
use xilem::{
    vello::{
        kurbo::{Affine, Point, RoundedRect, Size},
        peniko::{BlendMode, Color},
    },
    view::{Id, View, ViewMarker},
    widget::{ChangeFlags, Event, EventCx, Pod, Widget},
    MessageResult,
};

#[derive(Clone)]
pub struct ButtonStyle {
    height: f64,
    padding: f64,
    shape: f64,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            height: 40.,
            padding: 24.,
            shape: 20.,
        }
    }
}

pub struct Button<C> {
    content: C,
    style: ButtonStyle,
}

impl<C> Button<C> {
    pub fn new(content: C) -> Self {
        Self {
            content,
            style: ButtonStyle::default(),
        }
    }
}

impl<C> ViewMarker for Button<C> {}

impl<T, A, C> View<T, A> for Button<C>
where
    C: View<T, A>,
    C::Element: 'static,
{
    type State = ();

    type Element = ButtonWidget;

    fn build(&self, cx: &mut xilem::view::Cx) -> (Id, Self::State, Self::Element) {
        let (id, elem) = cx.with_new_id(|cx| {
            let (_, _, elem) = self.content.build(cx);
            ButtonWidget::new(elem, self.style.clone())
        });
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

pub struct ButtonWidget {
    content: Pod,
    style: ButtonStyle,
    ripple: RippleWidget,
}

impl ButtonWidget {
    pub fn new(content: impl Widget + 'static, style: ButtonStyle) -> Self {
        Self {
            content: Pod::new(content),
            style,
            ripple: RippleWidget::default(),
        }
    }
}

impl Widget for ButtonWidget {
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
        cx: &mut xilem::widget::LayoutCx,
        bc: &xilem::widget::BoxConstraints,
    ) -> xilem::vello::kurbo::Size {
        let content_size = self.content.layout(cx, bc);
        self.content.set_origin(
            cx,
            Point::new(
                self.style.padding,
                (self.style.height - content_size.height) / 2.,
            ),
        );
        Size::new(
            self.style.padding * 2. + content_size.width,
            self.style.height,
        )
    }

    fn accessibility(&mut self, _cx: &mut xilem::widget::AccessCx) {}

    fn paint(&mut self, cx: &mut xilem::widget::PaintCx, builder: &mut xilem::vello::SceneBuilder) {
        let rect = RoundedRect::new(0., 0., cx.size().width, cx.size().height, self.style.shape);
        builder.fill(
            xilem::vello::peniko::Fill::EvenOdd,
            Affine::default(),
            &Color::GREEN,
            None,
            &rect,
        );

        builder.push_layer(
            BlendMode::new(
                xilem::vello::peniko::Mix::Clip,
                xilem::vello::peniko::Compose::Plus,
            ),
            1.,
            Affine::default(),
            &rect,
        );

        self.ripple.paint(cx, builder);
        self.content.paint(cx, builder);

        builder.pop_layer()
    }
}
