use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
};

live_design!{
    import makepad_draw::shader::std::*;
    
    FoldButton= {{FoldButton}} {
        draw_bg: {
            instance opened: 0.0
            instance hover: 0.0
            
            uniform fade: 1.0
            
            fn pixel(self) -> vec4 {
                
                let sz = 3.;
                let c = vec2(5.0, 0.5 * self.rect_size.y);
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.clear(vec4(0.));
                // we have 3 points, and need to rotate around its center
                sdf.rotate(self.opened * 0.5 * PI + 0.5 * PI, c.x, c.y);
                sdf.move_to(c.x - sz, c.y + sz);
                sdf.line_to(c.x, c.y - sz);
                sdf.line_to(c.x + sz, c.y + sz);
                sdf.close_path();
                sdf.fill(mix(#a, #f, self.hover));
                return sdf.result * self.fade;
            }
        }
        
        abs_size: vec2(32, 12)
        abs_offset: vec2(4., 0.)
        
        walk: {
            width: 12,
            height: 12,
        }
        
        state: {
            
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 0.0}}
                }
                
                on = {
                    from: {all: Snap}
                    apply: {draw_bg: {hover: 1.0}}
                }
            }
            
            open = {
                default: yes
                no = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        opened: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                        draw_bg: {opened: (opened)}
                    }
                }
                yes = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        opened: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                        draw_bg: {opened: (opened)}
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct FoldButton {
    #[state] state: LiveState,
    
    #[live] opened: f32,
    
    #[live] draw_bg: DrawQuad,
    #[live] abs_size: DVec2,
    #[live] abs_offset: DVec2,
    #[live] walk: Walk,
}

impl LiveHook for FoldButton{
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx,FoldButton)
    }
}

#[derive(Clone, WidgetAction)]
pub enum FoldButtonAction {
    None,
    Opening,
    Closing,
    Animating(f32)
}

impl FoldButton {
    
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, FoldButtonAction),
    ) {
        if self.state_handle_event(cx, event).is_animating() {
            if self.state.is_track_animating(cx, id!(open)) {
                dispatch_action(cx, FoldButtonAction::Animating(self.opened))
            }
        };
        
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(_fe) => {
                if self.is_in_state(cx, id!(open.yes)) {
                    self.animate_state(cx, id!(open.no));
                    dispatch_action(cx, FoldButtonAction::Closing)
                }
                else {
                    self.animate_state(cx, id!(open.yes));
                    dispatch_action(cx, FoldButtonAction::Opening)
                }
                self.animate_state(cx, id!(hover.pressed));
            },
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                 self.animate_state(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animate_state(cx, id!(hover.off));
            }
            Hit::FingerUp(fe) => if fe.is_over {
                if fe.device.has_hovers() {
                    self.animate_state(cx, id!(hover.on));
                }
                else{
                    self.animate_state(cx, id!(hover.off));
                }
            }
            else {
                self.animate_state(cx, id!(hover.off));
            }
            _ => ()
        };
    }
    
    pub fn set_is_open(&mut self, cx: &mut Cx, is_open: bool, animate: Animate) {
        self.toggle_state(cx, is_open, animate, id!(open.yes), id!(open.no))
    }
    
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.draw_bg.draw_walk(cx, walk);
    }
    
    pub fn area(&mut self)->Area{
        self.draw_bg.area()
    }
    
    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2, fade: f64) {
        self.draw_bg.apply_over(cx, live!{fade: (fade)});
        self.draw_bg.draw_abs(cx, Rect {
            pos: pos + self.abs_offset,
            size: self.abs_size
        });
    }
}

impl Widget for FoldButton {
    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_bg.redraw(cx);
    }
    
    fn handle_widget_event_with(&mut self, cx: &mut Cx, event: &Event, dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut | cx, action | {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid))
        });
    }
    
    fn get_walk(&self) -> Walk {self.walk}
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}
