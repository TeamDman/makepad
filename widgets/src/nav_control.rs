use crate::makepad_draw::*;


live_design!{
    import makepad_draw::shader::std::*;
    
    DrawFocusRect= {{DrawFocusRect}} {
        fn pixel(self) -> vec4 {
            return #000f
        }
    }
    
    NavControl= {{NavControl}} {
        draw_label: {
            text_style: {
                font_size: 6
            },
            color: #a
        }
    }
}

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawFocusRect {
    #[deref] draw_super: DrawQuad,
}

#[derive(Live, LiveHook)]
pub struct NavControl {
    #[live] draw_list: DrawList2d,
    #[live] draw_focus: DrawFocusRect,
    #[live] draw_label: DrawText,
    #[rust] _recent_focus: Area,
}

impl NavControl {
    
    pub fn send_trigger_to_scroll_stack(cx: &mut Cx, stack:Vec<Area>){
        let mut prev_area = None;
        for next_area in stack{
            if let Some(prev_area) = prev_area{
                cx.send_trigger(prev_area, Trigger{
                    id:live_id!(scroll_focus_nav),
                    from:next_area
                });
            }
            prev_area = Some(next_area);
        }
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, root: DrawListId) {
        match event {
            Event::KeyDown(ke) => match ke.key_code {
                KeyCode::Tab => {
                    if ke.modifiers.shift {
                        let mut prev_area = Area::Empty;
                        if let Some((prev_area, scroll_stack)) = Cx2d::iterate_nav_stops(cx, root, | cx, stop | {
                            if cx.has_key_focus(stop.area) {
                                return Some(prev_area);
                            }
                            prev_area = stop.area;
                            None
                        }) {
                            if !prev_area.is_empty() {
                                Self::send_trigger_to_scroll_stack(cx, scroll_stack);
                                cx.set_key_focus(prev_area);
                            }
                        }
                    }
                    else {
                        let mut next_stop = false;
                        if let Some((next_area, scroll_stack)) = Cx2d::iterate_nav_stops(cx, root, | cx, stop | {
                            if next_stop {
                                return Some(stop.area)
                            }
                            if cx.has_key_focus(stop.area) {
                                next_stop = true;
                            }
                            None
                        }) {
                            Self::send_trigger_to_scroll_stack(cx, scroll_stack);
                            cx.set_key_focus(next_area);
                        }
                    }
                }
                _ => ()
            },
            _ => ()
        }
    }
    
    pub fn draw(&mut self, cx: &mut Cx2d) {
        if !self.draw_list.begin(cx, Walk::default()).is_redrawing() {
            return
        }
        
        self.draw_list.end(cx);
    }
}


