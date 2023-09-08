use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
    fold_button::*
};

live_design!{
    FoldHeaderBase = {{FoldHeader}} {}
}

#[derive(Live)]
pub struct FoldHeader {
    #[rust] draw_state: DrawStateWrap<DrawState>,
    #[rust] rect_size: f64,
    #[rust] area: Area,
    #[live] header: WidgetRef,
    #[live] body: WidgetRef,
    #[animator] animator: Animator,

    #[live] opened: f64,
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    #[live] body_walk: Walk,
}

impl LiveHook for FoldHeader{
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx,FoldHeader)
    }
}

#[derive(Clone)]
enum DrawState {
    DrawHeader,
    DrawBody
}

impl Widget for FoldHeader {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            if self.animator.is_track_animating(cx, id!(open)) {
                self.area.redraw(cx);
            }
        };
        
        for item in self.header.handle_widget_event(cx, event) {
            if item.widget_uid == self.header.widget(id!(fold_button)).widget_uid(){
                match item.action.cast() {
                    FoldButtonAction::Opening => {
                        self.animator_play(cx, id!(open.on))
                    }
                    FoldButtonAction::Closing => {
                        self.animator_play(cx, id!(open.off))
                    }
                    _ => ()
                }
            }
            dispatch_action(cx, item)
        }
        
        self.body.handle_widget_event_with(cx, event, dispatch_action);
    }
    
    fn redraw(&mut self, cx: &mut Cx) {
        self.header.redraw(cx);
        self.body.redraw(cx);
    }
    
    fn walk(&self) -> Walk {self.walk}

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.header.find_widgets(path, cached, results);
        self.body.find_widgets(path, cached, results);
    }
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        if self.draw_state.begin(cx, DrawState::DrawHeader) {
            cx.begin_turtle(walk, self.layout);
        }
        if let Some(DrawState::DrawHeader) = self.draw_state.get() {
            self.header.draw_widget(cx) ?;
            cx.begin_turtle(
                self.body_walk,
                Layout::flow_down()
                .with_scroll(dvec2(0.0, self.rect_size * (1.0 - self.opened)))
            );
            self.draw_state.set(DrawState::DrawBody);
        }
        if let Some(DrawState::DrawBody) = self.draw_state.get() {
            self.body.draw_widget(cx) ?;
            self.rect_size = cx.turtle().used().y;
            cx.end_turtle();
            cx.end_turtle_with_area(&mut self.area);
            self.draw_state.end();
        }
        WidgetDraw::done()
    }
}

#[derive(Clone, WidgetAction)]
pub enum FoldHeaderAction {
    Opening,
    Closing,
    None
}