#[derive(Default)]
pub enum AppState {
    #[default]
    Init,
    DaemonBootingUp {
        start_time: std::time::SystemTime,
    },
    ConnectingToDaemon,
    Running {
        socket: shared::networking::Socket<
            shared::networking::DaemonMessage,
            shared::networking::ClientMessage,
        >,
        sync_state: SyncState,
    },
}

#[derive(Default, PartialEq)]
pub enum SyncState {
    #[default]
    No,
    Requested,
    Yes,
}

#[derive(Debug, Default, Clone)]
pub enum BackgroundState {
    #[default]
    NotSent,
    Sent,
    Connected {
        id: shared::id::ID,
    },
}

#[derive(Debug, Clone)]
pub struct State<Inner> {
    pub str_anim: crate::animations::StringAnimation,
    pub inner: Inner,
}

impl State<AppState> {
    pub fn set_init(&mut self) {
        self.str_anim = crate::animations::StringAnimation::new(
            300,
            "Offline", // Initializing
            eframe::egui::Color32::RED,
        );
        self.inner = AppState::Init
    }
    pub fn set_daemon_booting_up(&mut self, start_time: std::time::SystemTime) {
        self.str_anim = crate::animations::StringAnimation::new(
            300,
            "Initializing", // Daemon is booting up
            eframe::egui::Color32::YELLOW,
        );
        self.inner = AppState::DaemonBootingUp { start_time }
    }

    pub fn set_connecting_to_daemon(&mut self) {
        self.str_anim = crate::animations::StringAnimation::new(
            100,
            "Connecting. . .",
            eframe::egui::Color32::YELLOW,
        );
        self.inner = AppState::ConnectingToDaemon
    }

    pub fn set_running(
        &mut self,
        socket: shared::networking::Socket<
            shared::networking::DaemonMessage,
            shared::networking::ClientMessage,
        >,
        sync_state: SyncState,
    ) {
        self.str_anim =
            crate::animations::StringAnimation::new(200, "Connected", eframe::egui::Color32::GREEN);
        self.inner = AppState::Running { socket, sync_state }
    }
    pub fn is_running(&self) -> bool {
        matches!(self.inner, AppState::Running { .. })
    }
    pub fn get_socket(
        &mut self,
    ) -> Option<
        &mut shared::networking::Socket<
            shared::networking::DaemonMessage,
            shared::networking::ClientMessage,
        >,
    > {
        if let AppState::Running { socket, .. } = &mut self.inner {
            Some(socket)
        } else {
            None
        }
    }
}

impl Default for State<AppState> {
    fn default() -> Self {
        let mut o = Self {
            str_anim: crate::animations::StringAnimation::new(
                0,
                "..",
                eframe::egui::Color32::TRANSPARENT,
            ),
            inner: AppState::Init,
        };
        o.set_init();
        o
    }
}

impl State<BackgroundState> {
    pub fn set_not_sent(&mut self) {
        self.str_anim = crate::animations::StringAnimation::new(
            300,
            "Not yet sent", // Initializing
            eframe::egui::Color32::RED,
        );
        self.inner = BackgroundState::NotSent
    }
    pub fn set_sent(&mut self) {
        self.str_anim = crate::animations::StringAnimation::new(
            100,
            "Sent, waiting for a response", // Initializing
            eframe::egui::Color32::YELLOW,
        );
        self.inner = BackgroundState::Sent
    }
    pub fn set_connected(&mut self, id: shared::id::ID) {
        self.str_anim = crate::animations::StringAnimation::new(
            100,
            "Connected", // Initializing
            eframe::egui::Color32::GREEN,
        );
        self.inner = BackgroundState::Connected { id }
    }
}

impl Default for State<BackgroundState> {
    fn default() -> Self {
        let mut o = Self {
            str_anim: crate::animations::StringAnimation::new(
                0,
                "..",
                eframe::egui::Color32::TRANSPARENT,
            ),
            inner: BackgroundState::NotSent,
        };
        o.set_not_sent();
        o
    }
}

impl SyncState {
    pub fn is_synched(&self) -> bool {
        matches!(self, SyncState::Yes)
    }

    pub fn is_requested(&self) -> bool {
        matches!(self, SyncState::Requested)
    }
}