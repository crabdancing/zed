use std::cmp;

use gpui::{
    AnyElement, App, BorderStyle, Bounds, Corners, Edges, Hsla, TextLayout, point, prelude::*, quad, size,
};
use ui::prelude::*;

pub struct CompletionDiffElement {
    element: AnyElement,
    text_layout: TextLayout,
    cursor_offset: usize,
}

impl CompletionDiffElement {
}

impl IntoElement for CompletionDiffElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for CompletionDiffElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (self.element.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(position) = self.text_layout.position_for_index(self.cursor_offset) {
            let bounds = self.text_layout.bounds();
            let line_height = self.text_layout.line_height();
            let line_width = self
                .text_layout
                .line_layout_for_index(self.cursor_offset)
                .map_or(bounds.size.width, |layout| layout.width());
            window.paint_quad(quad(
                Bounds::new(
                    point(bounds.origin.x, position.y),
                    size(cmp::max(bounds.size.width, line_width), line_height),
                ),
                Corners::default(),
                cx.theme().colors().editor_active_line_background,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));
            self.element.paint(window, cx);
            window.paint_quad(quad(
                Bounds::new(position, size(px(2.), line_height)),
                Corners::default(),
                cx.theme().players().local().cursor,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));
        }
    }
}
