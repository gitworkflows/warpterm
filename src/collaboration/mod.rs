use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, broadcast};
use serde::{Deserialize, Serialize};
use crate::error::WarpError;

pub mod session_manager;
pub mod real_time_sync;
pub mod voice_chat;
pub mod screen_sharing;
pub mod code_sharing;
pub mod whiteboard;
pub mod presence;
pub mod permissions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub participants: Vec<Participant>,
    pub session_type: SessionType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: SessionStatus,
    pub settings: SessionSettings,
    pub shared_resources: Vec<SharedResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: ParticipantRole,
    pub permissions: Vec<Permission>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub status: ParticipantStatus,
    pub cursor_position: Option<CursorPosition>,
    pub current_view: Option<ViewState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Debugging,
    CodeReview,
    PairProgramming,
    Testing,
    Design,
    Meeting,
    Training,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Paused,
    Ended,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Owner,
    Moderator,
    Contributor,
    Observer,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    ViewCode,
    EditCode,
    ExecuteCode,
    ViewDebugger,
    ControlDebugger,
    ViewTerminal,
    ControlTerminal,
    ShareScreen,
    UseVoiceChat,
    UseTextChat,
    ManageParticipants,
    ModifySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantStatus {
    Online,
    Away,
    Busy,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub selection_start: Option<Position>,
    pub selection_end: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub current_file: String,
    pub scroll_position: u32,
    pub zoom_level: f32,
    pub active_panel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    pub max_participants: u32,
    pub require_approval: bool,
    pub allow_anonymous: bool,
    pub enable_voice_chat: bool,
    pub enable_screen_sharing: bool,
    pub enable_file_sharing: bool,
    pub enable_whiteboard: bool,
    pub auto_save_interval: u64,
    pub session_timeout: u64,
    pub recording_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub name: String,
    pub path: String,
    pub owner_id: String,
    pub permissions: HashMap<String, Vec<Permission>>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub version: u64,
    pub locked_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    File,
    Directory,
    Terminal,
    Debugger,
    Whiteboard,
    Screen,
    Voice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEvent {
    pub event_id: String,
    pub session_id: String,
    pub user_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: EventType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Participant events
    ParticipantJoined,
    ParticipantLeft,
    ParticipantStatusChanged,
    CursorMoved,
    ViewChanged,
    
    // Code events
    CodeChanged,
    FileOpened,
    FileClosed,
    FileSaved,
    
    // Debug events
    BreakpointSet,
    BreakpointRemoved,
    DebuggerStarted,
    DebuggerStopped,
    DebuggerPaused,
    DebuggerResumed,
    
    // Communication events
    ChatMessage,
    VoiceStarted,
    VoiceStopped,
    ScreenShareStarted,
    ScreenShareStopped,
    
    // System events
    SessionStarted,
    SessionEnded,
    PermissionChanged,
    SettingsChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: String,
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub message_type: MessageType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub reply_to: Option<String>,
    pub reactions: HashMap<String, Vec<String>>, // emoji -> user_ids
    pub attachments: Vec<MessageAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Code,
    File,
    System,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub attachment_id: String,
    pub filename: String,
    pub file_type: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    pub change_id: String,
    pub file_path: String,
    pub user_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub change_type: ChangeType,
    pub start_position: Position,
    pub end_position: Position,
    pub old_content: String,
    pub new_content: String,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Insert,
    Delete,
    Replace,
    Move,
}

pub struct CollaborationManager {
    sessions: Arc<RwLock<HashMap<String, CollaborationSession>>>,
    session_manager: Arc<session_manager::SessionManager>,
    real_time_sync: Arc<real_time_sync::RealTimeSync>,
    voice_chat: Arc<voice_chat::VoiceChatManager>,
    screen_sharing: Arc<screen_sharing::ScreenSharingManager>,
    code_sharing: Arc<code_sharing::CodeSharingManager>,
    whiteboard: Arc<whiteboard::WhiteboardManager>,
    presence: Arc<presence::PresenceManager>,
    permissions: Arc<permissions::PermissionManager>,
    event_broadcaster: broadcast::Sender<CollaborationEvent>,
    active_connections: Arc<Mutex<HashMap<String, Vec<String>>>>, // session_id -> user_ids
}

impl CollaborationManager {
    pub async fn new() -> Result<Self, WarpError> {
        let (event_broadcaster, _) = broadcast::channel(1000);
        
        Ok(Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_manager: Arc::new(session_manager::SessionManager::new().await?),
            real_time_sync: Arc::new(real_time_sync::RealTimeSync::new().await?),
            voice_chat: Arc::new(voice_chat::VoiceChatManager::new().await?),
            screen_sharing: Arc::new(screen_sharing::ScreenSharingManager::new().await?),
            code_sharing: Arc::new(code_sharing::CodeSharingManager::new().await?),
            whiteboard: Arc::new(whiteboard::WhiteboardManager::new().await?),
            presence: Arc::new(presence::PresenceManager::new().await?),
            permissions: Arc::new(permissions::PermissionManager::new().await?),
            event_broadcaster,
            active_connections: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn create_session(&self, owner_id: &str, session_type: SessionType, settings: SessionSettings) -> Result<String, WarpError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let session = CollaborationSession {
            session_id: session_id.clone(),
            name: format!("{:?} Session", session_type),
            description: String::new(),
            owner_id: owner_id.to_string(),
            participants: vec![Participant {
                user_id: owner_id.to_string(),
                username: owner_id.to_string(), // Would be fetched from user service
                display_name: owner_id.to_string(),
                avatar_url: None,
                role: ParticipantRole::Owner,
                permissions: vec![
                    Permission::ViewCode,
                    Permission::EditCode,
                    Permission::ExecuteCode,
                    Permission::ViewDebugger,
                    Permission::ControlDebugger,
                    Permission::ViewTerminal,
                    Permission::ControlTerminal,
                    Permission::ShareScreen,
                    Permission::UseVoiceChat,
                    Permission::UseTextChat,
                    Permission::ManageParticipants,
                    Permission::ModifySettings,
                ],
                joined_at: chrono::Utc::now(),
                last_active: chrono::Utc::now(),
                status: ParticipantStatus::Online,
                cursor_position: None,
                current_view: None,
            }],
            session_type,
            created_at: chrono::Utc::now(),
            expires_at: None,
            status: SessionStatus::Active,
            settings,
            shared_resources: Vec::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session.clone());

        // Initialize session components
        self.session_manager.initialize_session(&session).await?;
        self.real_time_sync.create_sync_room(&session_id).await?;
        
        // Broadcast session created event
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.clone(),
            user_id: owner_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::SessionStarted,
            data: serde_json::to_value(&session)?,
        };
        let _ = self.event_broadcaster.send(event);

        Ok(session_id)
    }

    pub async fn join_session(&self, session_id: &str, user_id: &str, role: ParticipantRole) -> Result<(), WarpError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            // Check if user is already in session
            if session.participants.iter().any(|p| p.user_id == user_id) {
                return Err(WarpError::ConfigError("User already in session".to_string()));
            }

            // Check session limits
            if session.participants.len() >= session.settings.max_participants as usize {
                return Err(WarpError::ConfigError("Session is full".to_string()));
            }

            // Add participant
            let participant = Participant {
                user_id: user_id.to_string(),
                username: user_id.to_string(), // Would be fetched from user service
                display_name: user_id.to_string(),
                avatar_url: None,
                role: role.clone(),
                permissions: self.permissions.get_default_permissions(&role).await?,
                joined_at: chrono::Utc::now(),
                last_active: chrono::Utc::now(),
                status: ParticipantStatus::Online,
                cursor_position: None,
                current_view: None,
            };

            session.participants.push(participant.clone());

            // Update active connections
            let mut connections = self.active_connections.lock().await;
            connections.entry(session_id.to_string()).or_insert_with(Vec::new).push(user_id.to_string());

            // Join real-time sync room
            self.real_time_sync.join_room(session_id, user_id).await?;

            // Update presence
            self.presence.set_user_online(user_id, session_id).await?;

            // Broadcast participant joined event
            let event = CollaborationEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: chrono::Utc::now(),
                event_type: EventType::ParticipantJoined,
                data: serde_json::to_value(&participant)?,
            };
            let _ = self.event_broadcaster.send(event);

            Ok(())
        } else {
            Err(WarpError::ConfigError("Session not found".to_string()))
        }
    }

    pub async fn leave_session(&self, session_id: &str, user_id: &str) -> Result<(), WarpError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            // Remove participant
            session.participants.retain(|p| p.user_id != user_id);

            // Update active connections
            let mut connections = self.active_connections.lock().await;
            if let Some(user_list) = connections.get_mut(session_id) {
                user_list.retain(|id| id != user_id);
            }

            // Leave real-time sync room
            self.real_time_sync.leave_room(session_id, user_id).await?;

            // Update presence
            self.presence.set_user_offline(user_id, session_id).await?;

            // Stop any active sharing
            self.voice_chat.stop_for_user(session_id, user_id).await?;
            self.screen_sharing.stop_for_user(session_id, user_id).await?;

            // Broadcast participant left event
            let event = CollaborationEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: chrono::Utc::now(),
                event_type: EventType::ParticipantLeft,
                data: serde_json::json!({"user_id": user_id}),
            };
            let _ = self.event_broadcaster.send(event);

            // End session if no participants left
            if session.participants.is_empty() {
                self.end_session(session_id).await?;
            }

            Ok(())
        } else {
            Err(WarpError::ConfigError("Session not found".to_string()))
        }
    }

    pub async fn send_chat_message(&self, session_id: &str, user_id: &str, content: &str, message_type: MessageType) -> Result<String, WarpError> {
        let message_id = uuid::Uuid::new_v4().to_string();
        
        let message = ChatMessage {
            message_id: message_id.clone(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            username: user_id.to_string(), // Would be fetched from user service
            content: content.to_string(),
            message_type,
            timestamp: chrono::Utc::now(),
            reply_to: None,
            reactions: HashMap::new(),
            attachments: Vec::new(),
        };

        // Store message
        self.session_manager.store_chat_message(&message).await?;

        // Broadcast message
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::ChatMessage,
            data: serde_json::to_value(&message)?,
        };
        let _ = self.event_broadcaster.send(event);

        Ok(message_id)
    }

    pub async fn share_code(&self, session_id: &str, user_id: &str, file_path: &str, content: &str) -> Result<(), WarpError> {
        // Check permissions
        if !self.permissions.has_permission(session_id, user_id, &Permission::EditCode).await? {
            return Err(WarpError::ConfigError("Insufficient permissions".to_string()));
        }

        self.code_sharing.share_file(session_id, user_id, file_path, content).await?;

        // Broadcast file opened event
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::FileOpened,
            data: serde_json::json!({
                "file_path": file_path,
                "content": content
            }),
        };
        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    pub async fn apply_code_change(&self, session_id: &str, user_id: &str, change: CodeChange) -> Result<(), WarpError> {
        // Check permissions
        if !self.permissions.has_permission(session_id, user_id, &Permission::EditCode).await? {
            return Err(WarpError::ConfigError("Insufficient permissions".to_string()));
        }

        // Apply change through real-time sync
        self.real_time_sync.apply_change(session_id, &change).await?;

        // Broadcast code change event
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::CodeChanged,
            data: serde_json::to_value(&change)?,
        };
        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    pub async fn start_voice_chat(&self, session_id: &str, user_id: &str) -> Result<String, WarpError> {
        // Check permissions
        if !self.permissions.has_permission(session_id, user_id, &Permission::UseVoiceChat).await? {
            return Err(WarpError::ConfigError("Insufficient permissions".to_string()));
        }

        let room_id = self.voice_chat.start_voice_chat(session_id, user_id).await?;

        // Broadcast voice started event
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::VoiceStarted,
            data: serde_json::json!({"room_id": room_id}),
        };
        let _ = self.event_broadcaster.send(event);

        Ok(room_id)
    }

    pub async fn start_screen_sharing(&self, session_id: &str, user_id: &str) -> Result<String, WarpError> {
        // Check permissions
        if !self.permissions.has_permission(session_id, user_id, &Permission::ShareScreen).await? {
            return Err(WarpError::ConfigError("Insufficient permissions".to_string()));
        }

        let stream_id = self.screen_sharing.start_screen_share(session_id, user_id).await?;

        // Broadcast screen share started event
        let event = CollaborationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::ScreenShareStarted,
            data: serde_json::json!({"stream_id": stream_id}),
        };
        let _ = self.event_broadcaster.send(event);

        Ok(stream_id)
    }

    pub async fn update_cursor_position(&self, session_id: &str, user_id: &str, position: CursorPosition) -> Result<(), WarpError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if let Some(participant) = session.participants.iter_mut().find(|p| p.user_id == user_id) {
                participant.cursor_position = Some(position.clone());
                participant.last_active = chrono::Utc::now();

                // Broadcast cursor moved event
                let event = CollaborationEvent {
                    event_id: uuid::Uuid::new_v4().to_string(),
                    session_id: session_id.to_string(),
                    user_id: user_id.to_string(),
                    timestamp: chrono::Utc::now(),
                    event_type: EventType::CursorMoved,
                    data: serde_json::to_value(&position)?,
                };
                let _ = self.event_broadcaster.send(event);
            }
        }

        Ok(())
    }

    pub async fn get_session(&self, session_id: &str) -> Result<CollaborationSession, WarpError> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id)
            .cloned()
            .ok_or_else(|| WarpError::ConfigError("Session not found".to_string()))
    }

    pub async fn get_active_sessions(&self, user_id: &str) -> Result<Vec<CollaborationSession>, WarpError> {
        let sessions = self.sessions.read().await;
        let active_sessions: Vec<CollaborationSession> = sessions
            .values()
            .filter(|session| {
                session.status == SessionStatus::Active &&
                session.participants.iter().any(|p| p.user_id == user_id)
            })
            .cloned()
            .collect();

        Ok(active_sessions)
    }

    pub async fn end_session(&self, session_id: &str) -> Result<(), WarpError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = SessionStatus::Ended;

            // Clean up resources
            self.real_time_sync.cleanup_room(session_id).await?;
            self.voice_chat.cleanup_session(session_id).await?;
            self.screen_sharing.cleanup_session(session_id).await?;
            self.code_sharing.cleanup_session(session_id).await?;
            self.whiteboard.cleanup_session(session_id).await?;

            // Clear active connections
            let mut connections = self.active_connections.lock().await;
            connections.remove(session_id);

            // Broadcast session ended event
            let event = CollaborationEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                user_id: session.owner_id.clone(),
                timestamp: chrono::Utc::now(),
                event_type: EventType::SessionEnded,
                data: serde_json::json!({"session_id": session_id}),
            };
            let _ = self.event_broadcaster.send(event);

            // Archive session
            self.session_manager.archive_session(session).await?;
            sessions.remove(session_id);
        }

        Ok(())
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<CollaborationEvent> {
        self.event_broadcaster.subscribe()
    }
}
