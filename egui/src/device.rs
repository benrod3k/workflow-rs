use crate::imports::*;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    #[default]
    Landscape,
    Portrait,
}

#[derive(Default, Clone)]
pub struct Device {
    pub mobile_device: bool,
    pub mobile_forced: bool,
    pub orientation: Orientation,
    pub orientation_forced: Option<Orientation>,
    pub screen_size: Vec2,
}

impl Device {
    pub fn set_screen_size(&mut self, rect: &Rect) {
        let size = rect.size();

        if size.x * 3. < size.y * 2. || size.x < 540.0 {
            self.orientation = Orientation::Portrait;
        } else {
            self.orientation = Orientation::Landscape;
        }

        self.screen_size = rect.size();
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation_forced.unwrap_or(self.orientation)
    }

    pub fn is_mobile(&self) -> bool {
        self.mobile_device || self.mobile_forced
    }

    pub fn toggle_portrait(&mut self) {
        if self.orientation_forced.is_none() {
            self.orientation_forced = Some(Orientation::Portrait);
        } else {
            self.orientation_forced = None;
        }
    }

    pub fn toggle_mobile(&mut self) {
        self.mobile_forced = !self.mobile_forced;
    }

    pub fn is_single_pane(&self) -> bool {
        workflow_core::runtime::is_chrome_extension()
            || self.mobile_forced
            || self.mobile_device
            || self.orientation() == Orientation::Portrait
    }

    #[inline]
    pub fn is_desktop(&self) -> bool {
        !self.is_single_pane()
    }

    pub fn force_orientation(&mut self, orientation: Option<Orientation>) {
        self.orientation_forced = orientation;
    }
}
