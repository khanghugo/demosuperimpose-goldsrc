pub struct GhostFrame {
    pub origin: [f32; 3],
    pub viewangles: [f32; 3],
    pub sequence: Option<Vec<u8>>,
    pub frame: Option<Vec<u8>>,
    pub animtime: Option<Vec<u8>>,
}

pub struct GhostInfo {
    pub ghost_name: String,
    pub entity_index: u16,
    pub frames: Vec<GhostFrame>,
    pub ghost_anim_frame: f32,
}

impl GhostInfo {
    pub fn new() -> Self {
        Self {
            ghost_name: "".to_string(),
            entity_index: 0,
            frames: vec![],
            ghost_anim_frame: 0.,
        }
    }

    pub fn append_frame(
        &mut self,
        origin: [f32; 3],
        viewangles: [f32; 3],
        sequence: Option<Vec<u8>>,
        frame: Option<Vec<u8>>,
        animtime: Option<Vec<u8>>,
    ) {
        self.frames.push(GhostFrame {
            origin,
            viewangles,
            sequence,
            frame,
            animtime,
        });
    }

    pub fn get_frame(&self, idx: usize) -> &GhostFrame {
        // Eh
        self.frames.get(idx).unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.frames.len()
    }

    pub fn set_name(&mut self, name: String) {
        self.ghost_name = name.to_owned();
    }

    pub fn get_name(&self) -> String {
        self.ghost_name.to_owned()
    }

    pub fn set_entity_index(&mut self, idx: u16) {
        self.entity_index = idx;
    }

    pub fn get_entity_index(&self) -> u16 {
        self.entity_index
    }

    pub fn increment_ghost_anim_frame(&mut self) {
        self.ghost_anim_frame += 1.;
    }

    pub fn reset_ghost_anim_frame(&mut self) {
        self.ghost_anim_frame = 0.;
    }

    pub fn get_ghost_anim_frame(&self) -> f32 {
        self.ghost_anim_frame
    }
}
