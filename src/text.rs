use std::borrow::Cow;
use xilem::parley::{self, FontContext, Layout, LayoutContext};
use xilem::text::{render_text, ParleyBrush};
use xilem::vello::kurbo::{Affine, Size};
use xilem::vello::peniko::{Brush, Color};
use xilem::vello::SceneBuilder;
use xilem::view::{Cx, Id, View, ViewMarker};
use xilem::widget::{
    AccessCx, BoxConstraints, ChangeFlags, Event, EventCx, LayoutCx, LifeCycle, LifeCycleCx,
    PaintCx, UpdateCx, Widget,
};
use xilem::{accesskit, Axis, MessageResult};

#[derive(Clone)]
pub struct TextStyle {
    font_size: f64,
}

pub struct TextBuilder {
    content: Option<Cow<'static, str>>,
    style: TextStyle,
}

impl TextBuilder {
    pub fn content(&mut self, content: impl Into<Cow<'static, str>>) -> &mut Self {
        self.content = Some(content.into());
        self
    }

    pub fn font_size(&mut self, font_size: f64) -> &mut Self {
        self.style.font_size = font_size;
        self
    }

    pub fn build(&mut self) -> Text {
        Text {
            content: self.content.take().unwrap_or_else(|| Cow::Borrowed("")),
            style: self.style.clone(),
        }
    }
}

pub struct Text {
    content: Cow<'static, str>,
    style: TextStyle,
}

impl Text {
    pub fn new(_content: impl Into<Cow<'static, str>>) -> Self {
        Self::builder().build()
    }

    pub fn builder() -> TextBuilder {
        TextBuilder {
            content: None,
            style: TextStyle { font_size: 12. },
        }
    }
}

impl ViewMarker for Text {}

impl<T, A> View<T, A> for Text {
    type State = ();

    type Element = TextWidget;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, element) =
            cx.with_new_id(|_| TextWidget::new(self.content.clone(), self.style.clone()));
        (id, (), element)
    }

    fn rebuild(
        &self,
        _cx: &mut Cx,
        prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        if self.content != prev.content {
            element.set_text(self.content.clone())
        } else {
            ChangeFlags::empty()
        }
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Stale(message)
    }
}

pub struct TextWidget {
    text: Cow<'static, str>,
    layout: Option<Layout<ParleyBrush>>,
    style: TextStyle,
}

impl TextWidget {
    pub fn new(text: Cow<'static, str>, style: TextStyle) -> TextWidget {
        TextWidget {
            text,
            layout: None,
            style,
        }
    }

    pub fn set_text(&mut self, text: Cow<'static, str>) -> ChangeFlags {
        self.text = text;
        self.layout = None;
        ChangeFlags::LAYOUT | ChangeFlags::PAINT
    }

    fn get_layout_mut(&mut self, font_cx: &mut FontContext) -> &mut Layout<ParleyBrush> {
        if self.layout.is_none() {
            let mut lcx = LayoutContext::new();
            let mut layout_builder = lcx.ranged_builder(font_cx, &self.text, 1.0);
            layout_builder.push_default(&parley::style::StyleProperty::FontSize(
                self.style.font_size as _,
            ));
            layout_builder.push_default(&parley::style::StyleProperty::Brush(ParleyBrush(
                Brush::Solid(Color::rgb8(255, 255, 255)),
            )));
            self.layout = Some(layout_builder.build());
        }

        self.layout.as_mut().unwrap()
    }

    fn layout_text(&mut self, font_cx: &mut FontContext, bc: &BoxConstraints) -> Size {
        let max_advance = if bc.max().width.is_finite() {
            Some(bc.max().width as f32)
        } else if bc.min().width.is_sign_negative() {
            Some(0.0)
        } else {
            None
        };

        let layout = self.get_layout_mut(font_cx);
        layout.break_all_lines(max_advance, parley::layout::Alignment::Start);

        Size {
            width: layout.width() as f64,
            height: layout.height() as f64,
        }
    }
}

impl Widget for TextWidget {
    fn event(&mut self, _cx: &mut EventCx, _event: &Event) {}

    fn lifecycle(&mut self, _cx: &mut LifeCycleCx, _event: &LifeCycle) {}

    fn update(&mut self, cx: &mut UpdateCx) {
        cx.request_layout();
    }

    fn compute_max_intrinsic(&mut self, axis: Axis, cx: &mut LayoutCx, bc: &BoxConstraints) -> f64 {
        let size = self.layout_text(cx.font_cx(), bc);
        match axis {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    fn layout(&mut self, cx: &mut LayoutCx, bc: &BoxConstraints) -> Size {
        cx.request_paint();
        self.layout_text(cx.font_cx(), bc)
    }

    fn paint(&mut self, _cx: &mut PaintCx, builder: &mut SceneBuilder) {
        if let Some(layout) = &self.layout {
            render_text(builder, Affine::IDENTITY, layout);
        }
    }

    fn accessibility(&mut self, cx: &mut AccessCx) {
        let mut builder = accesskit::NodeBuilder::new(accesskit::Role::StaticText);
        builder.set_value(self.text.clone());
        cx.push_node(builder);
    }
}
