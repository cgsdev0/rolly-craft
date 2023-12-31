use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Achievement {
    pub id: String,
    pub progress: i64,
    pub rd: Option<i64>,
    pub rn: Option<i64>,
    pub unlocked: String,
}
impl From<&Achievement> for Achievement {
    fn from(value: &Achievement) -> Self {
        value.clone()
    }
}
impl Achievement {
    pub fn builder() -> builder::Achievement {
        builder::Achievement::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementData {
    pub description: String,
    pub id: String,
    pub image_url: Option<String>,
    pub max_progress: i64,
    pub name: String,
}
impl From<&AchievementData> for AchievementData {
    fn from(value: &AchievementData) -> Self {
        value.clone()
    }
}
impl AchievementData {
    pub fn builder() -> builder::AchievementData {
        builder::AchievementData::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementProgress {
    pub achievement_id: String,
    pub progress: i64,
    #[serde(rename = "type")]
    pub type_: AchievementProgressType,
    pub user_id: UserId,
    pub user_index: i64,
}
impl From<&AchievementProgress> for AchievementProgress {
    fn from(value: &AchievementProgress) -> Self {
        value.clone()
    }
}
impl AchievementProgress {
    pub fn builder() -> builder::AchievementProgress {
        builder::AchievementProgress::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AchievementProgressType {
    #[serde(rename = "achievement_progress")]
    AchievementProgress,
}
impl From<&AchievementProgressType> for AchievementProgressType {
    fn from(value: &AchievementProgressType) -> Self {
        value.clone()
    }
}
impl ToString for AchievementProgressType {
    fn to_string(&self) -> String {
        match *self {
            Self::AchievementProgress => "achievement_progress".to_string(),
        }
    }
}
impl std::str::FromStr for AchievementProgressType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "achievement_progress" => Ok(Self::AchievementProgress),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AchievementProgressType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AchievementProgressType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AchievementProgressType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for AchievementProgressType {
    fn default() -> Self {
        AchievementProgressType::AchievementProgress
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AchievementUnlock {
    pub description: String,
    pub id: String,
    pub image_url: Option<String>,
    pub max_progress: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: AchievementUnlockType,
    pub user_id: uuid::Uuid,
    pub user_index: i64,
}
impl From<&AchievementUnlock> for AchievementUnlock {
    fn from(value: &AchievementUnlock) -> Self {
        value.clone()
    }
}
impl AchievementUnlock {
    pub fn builder() -> builder::AchievementUnlock {
        builder::AchievementUnlock::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AchievementUnlockType {
    #[serde(rename = "achievement_unlock")]
    AchievementUnlock,
}
impl From<&AchievementUnlockType> for AchievementUnlockType {
    fn from(value: &AchievementUnlockType) -> Self {
        value.clone()
    }
}
impl ToString for AchievementUnlockType {
    fn to_string(&self) -> String {
        match *self {
            Self::AchievementUnlock => "achievement_unlock".to_string(),
        }
    }
}
impl std::str::FromStr for AchievementUnlockType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "achievement_unlock" => Ok(Self::AchievementUnlock),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AchievementUnlockType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AchievementUnlockType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AchievementUnlockType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for AchievementUnlockType {
    fn default() -> Self {
        AchievementUnlockType::AchievementUnlock
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthDonateResponse {
    pub link: String,
}
impl From<&AuthDonateResponse> for AuthDonateResponse {
    fn from(value: &AuthDonateResponse) -> Self {
        value.clone()
    }
}
impl AuthDonateResponse {
    pub fn builder() -> builder::AuthDonateResponse {
        builder::AuthDonateResponse::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthRefreshTokenResponse {
    pub access_token: String,
}
impl From<&AuthRefreshTokenResponse> for AuthRefreshTokenResponse {
    fn from(value: &AuthRefreshTokenResponse) -> Self {
        value.clone()
    }
}
impl AuthRefreshTokenResponse {
    pub fn builder() -> builder::AuthRefreshTokenResponse {
        builder::AuthRefreshTokenResponse::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "setting", deny_unknown_fields)]
pub enum AuthSettingsRequest {
    Color { color: Color },
    DiceType { dice_type: DiceType },
    Pubkey { text: String },
}
impl From<&AuthSettingsRequest> for AuthSettingsRequest {
    fn from(value: &AuthSettingsRequest) -> Self {
        value.clone()
    }
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChatMsg {
    pub msg: String,
    #[serde(rename = "type")]
    pub type_: ChatMsgType,
}
impl From<&ChatMsg> for ChatMsg {
    fn from(value: &ChatMsg) -> Self {
        value.clone()
    }
}
impl ChatMsg {
    pub fn builder() -> builder::ChatMsg {
        builder::ChatMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ChatMsgType {
    #[serde(rename = "chat")]
    Chat,
}
impl From<&ChatMsgType> for ChatMsgType {
    fn from(value: &ChatMsgType) -> Self {
        value.clone()
    }
}
impl ToString for ChatMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Chat => "chat".to_string(),
        }
    }
}
impl std::str::FromStr for ChatMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "chat" => Ok(Self::Chat),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ChatMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ChatMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ChatMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for ChatMsgType {
    fn default() -> Self {
        ChatMsgType::Chat
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Color {
    pub hue: f64,
    pub sat: f64,
}
impl From<&Color> for Color {
    fn from(value: &Color) -> Self {
        value.clone()
    }
}
impl Color {
    pub fn builder() -> builder::Color {
        builder::Color::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dice {
    #[serde(rename = "type")]
    pub type_: DiceType,
}
impl From<&Dice> for Dice {
    fn from(value: &Dice) -> Self {
        value.clone()
    }
}
impl Dice {
    pub fn builder() -> builder::Dice {
        builder::Dice::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DiceType {
    D6,
    D20,
}
impl From<&DiceType> for DiceType {
    fn from(value: &DiceType) -> Self {
        value.clone()
    }
}
impl ToString for DiceType {
    fn to_string(&self) -> String {
        match *self {
            Self::D6 => "D6".to_string(),
            Self::D20 => "D20".to_string(),
        }
    }
}
impl std::str::FromStr for DiceType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "D6" => Ok(Self::D6),
            "D20" => Ok(Self::D20),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DiceType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiceType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiceType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DieRoll {
    pub used: bool,
    pub value: i64,
}
impl From<&DieRoll> for DieRoll {
    fn from(value: &DieRoll) -> Self {
        value.clone()
    }
}
impl DieRoll {
    pub fn builder() -> builder::DieRoll {
        builder::DieRoll::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisconnectMsg {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: DisconnectMsgType,
}
impl From<&DisconnectMsg> for DisconnectMsg {
    fn from(value: &DisconnectMsg) -> Self {
        value.clone()
    }
}
impl DisconnectMsg {
    pub fn builder() -> builder::DisconnectMsg {
        builder::DisconnectMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DisconnectMsgType {
    #[serde(rename = "disconnect")]
    Disconnect,
}
impl From<&DisconnectMsgType> for DisconnectMsgType {
    fn from(value: &DisconnectMsgType) -> Self {
        value.clone()
    }
}
impl ToString for DisconnectMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Disconnect => "disconnect".to_string(),
        }
    }
}
impl std::str::FromStr for DisconnectMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "disconnect" => Ok(Self::Disconnect),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DisconnectMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DisconnectMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DisconnectMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for DisconnectMsgType {
    fn default() -> Self {
        DisconnectMsgType::Disconnect
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameError {
    pub error: String,
    #[serde(rename = "type")]
    pub type_: GameErrorType,
}
impl From<&GameError> for GameError {
    fn from(value: &GameError) -> Self {
        value.clone()
    }
}
impl GameError {
    pub fn builder() -> builder::GameError {
        builder::GameError::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GameErrorType {
    #[serde(rename = "error")]
    Error,
}
impl From<&GameErrorType> for GameErrorType {
    fn from(value: &GameErrorType) -> Self {
        value.clone()
    }
}
impl ToString for GameErrorType {
    fn to_string(&self) -> String {
        match *self {
            Self::Error => "error".to_string(),
        }
    }
}
impl std::str::FromStr for GameErrorType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GameErrorType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GameErrorType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GameErrorType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for GameErrorType {
    fn default() -> Self {
        GameErrorType::Error
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameState {
    #[serde(rename = "chatLog")]
    pub chat_log: Vec<String>,
    pub players: Vec<ServerPlayer>,
    #[serde(rename = "privateSession")]
    pub private_session: bool,
    pub rolled: bool,
    pub rolls: Vec<i64>,
    pub spectators: i64,
    pub turn_index: i64,
    #[serde(rename = "type")]
    pub type_: GameStateType,
    pub used: Vec<bool>,
    pub victory: bool,
}
impl From<&GameState> for GameState {
    fn from(value: &GameState) -> Self {
        value.clone()
    }
}
impl GameState {
    pub fn builder() -> builder::GameState {
        builder::GameState::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GameStateType {
    #[serde(rename = "game_state")]
    GameState,
}
impl From<&GameStateType> for GameStateType {
    fn from(value: &GameStateType) -> Self {
        value.clone()
    }
}
impl ToString for GameStateType {
    fn to_string(&self) -> String {
        match *self {
            Self::GameState => "game_state".to_string(),
        }
    }
}
impl std::str::FromStr for GameStateType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "game_state" => Ok(Self::GameState),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GameStateType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GameStateType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GameStateType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for GameStateType {
    fn default() -> Self {
        GameStateType::GameState
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IGameState {
    #[serde(rename = "chatLog")]
    pub chat_log: Vec<String>,
    pub players: Vec<Player>,
    #[serde(rename = "privateSession")]
    pub private_session: bool,
    pub rolled: bool,
    pub rolls: Vec<i64>,
    pub spectators: i64,
    pub turn_index: i64,
    pub used: Vec<bool>,
    pub victory: bool,
}
impl From<&IGameState> for IGameState {
    fn from(value: &IGameState) -> Self {
        value.clone()
    }
}
impl IGameState {
    pub fn builder() -> builder::IGameState {
        builder::IGameState::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JoinMsg {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: JoinMsgType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl From<&JoinMsg> for JoinMsg {
    fn from(value: &JoinMsg) -> Self {
        value.clone()
    }
}
impl JoinMsg {
    pub fn builder() -> builder::JoinMsg {
        builder::JoinMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JoinMsgType {
    #[serde(rename = "join")]
    Join,
}
impl From<&JoinMsgType> for JoinMsgType {
    fn from(value: &JoinMsgType) -> Self {
        value.clone()
    }
}
impl ToString for JoinMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Join => "join".to_string(),
        }
    }
}
impl std::str::FromStr for JoinMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "join" => Ok(Self::Join),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for JoinMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for JoinMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for JoinMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for JoinMsgType {
    fn default() -> Self {
        JoinMsgType::Join
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KickMsg {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: KickMsgType,
}
impl From<&KickMsg> for KickMsg {
    fn from(value: &KickMsg) -> Self {
        value.clone()
    }
}
impl KickMsg {
    pub fn builder() -> builder::KickMsg {
        builder::KickMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum KickMsgType {
    #[serde(rename = "kick")]
    Kick,
}
impl From<&KickMsgType> for KickMsgType {
    fn from(value: &KickMsgType) -> Self {
        value.clone()
    }
}
impl ToString for KickMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Kick => "kick".to_string(),
        }
    }
}
impl std::str::FromStr for KickMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "kick" => Ok(Self::Kick),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for KickMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for KickMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for KickMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for KickMsgType {
    fn default() -> Self {
        KickMsgType::Kick
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crowned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub score: i64,
    pub skip_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    pub win_count: i64,
}
impl From<&Player> for Player {
    fn from(value: &Player) -> Self {
        value.clone()
    }
}
impl Player {
    pub fn builder() -> builder::Player {
        builder::Player::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconnectMsg {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: ReconnectMsgType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl From<&ReconnectMsg> for ReconnectMsg {
    fn from(value: &ReconnectMsg) -> Self {
        value.clone()
    }
}
impl ReconnectMsg {
    pub fn builder() -> builder::ReconnectMsg {
        builder::ReconnectMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReconnectMsgType {
    #[serde(rename = "reconnect")]
    Reconnect,
}
impl From<&ReconnectMsgType> for ReconnectMsgType {
    fn from(value: &ReconnectMsgType) -> Self {
        value.clone()
    }
}
impl ToString for ReconnectMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Reconnect => "reconnect".to_string(),
        }
    }
}
impl std::str::FromStr for ReconnectMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "reconnect" => Ok(Self::Reconnect),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ReconnectMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReconnectMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReconnectMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for ReconnectMsgType {
    fn default() -> Self {
        ReconnectMsgType::Reconnect
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Redirect {
    pub room: String,
    #[serde(rename = "type")]
    pub type_: RedirectType,
}
impl From<&Redirect> for Redirect {
    fn from(value: &Redirect) -> Self {
        value.clone()
    }
}
impl Redirect {
    pub fn builder() -> builder::Redirect {
        builder::Redirect::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RedirectType {
    #[serde(rename = "redirect")]
    Redirect,
}
impl From<&RedirectType> for RedirectType {
    fn from(value: &RedirectType) -> Self {
        value.clone()
    }
}
impl ToString for RedirectType {
    fn to_string(&self) -> String {
        match *self {
            Self::Redirect => "redirect".to_string(),
        }
    }
}
impl std::str::FromStr for RedirectType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "redirect" => Ok(Self::Redirect),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RedirectType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RedirectType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RedirectType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RedirectType {
    fn default() -> Self {
        RedirectType::Redirect
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RefetchPlayerMsg {
    #[serde(rename = "type")]
    pub type_: RefetchPlayerMsgType,
    pub user_id: String,
}
impl From<&RefetchPlayerMsg> for RefetchPlayerMsg {
    fn from(value: &RefetchPlayerMsg) -> Self {
        value.clone()
    }
}
impl RefetchPlayerMsg {
    pub fn builder() -> builder::RefetchPlayerMsg {
        builder::RefetchPlayerMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RefetchPlayerMsgType {
    #[serde(rename = "refetch_player")]
    RefetchPlayer,
}
impl From<&RefetchPlayerMsgType> for RefetchPlayerMsgType {
    fn from(value: &RefetchPlayerMsgType) -> Self {
        value.clone()
    }
}
impl ToString for RefetchPlayerMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::RefetchPlayer => "refetch_player".to_string(),
        }
    }
}
impl std::str::FromStr for RefetchPlayerMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "refetch_player" => Ok(Self::RefetchPlayer),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RefetchPlayerMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RefetchPlayerMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RefetchPlayerMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RefetchPlayerMsgType {
    fn default() -> Self {
        RefetchPlayerMsgType::RefetchPlayer
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReportStats {
    pub doubles: i64,
    pub games: i64,
    pub rolls: i64,
    pub user_id: UserId,
    pub wins: i64,
}
impl From<&ReportStats> for ReportStats {
    fn from(value: &ReportStats) -> Self {
        value.clone()
    }
}
impl ReportStats {
    pub fn builder() -> builder::ReportStats {
        builder::ReportStats::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartMsg {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: RestartMsgType,
}
impl From<&RestartMsg> for RestartMsg {
    fn from(value: &RestartMsg) -> Self {
        value.clone()
    }
}
impl RestartMsg {
    pub fn builder() -> builder::RestartMsg {
        builder::RestartMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RestartMsgType {
    #[serde(rename = "restart")]
    Restart,
}
impl From<&RestartMsgType> for RestartMsgType {
    fn from(value: &RestartMsgType) -> Self {
        value.clone()
    }
}
impl ToString for RestartMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Restart => "restart".to_string(),
        }
    }
}
impl std::str::FromStr for RestartMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "restart" => Ok(Self::Restart),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RestartMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RestartMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RestartMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RestartMsgType {
    fn default() -> Self {
        RestartMsgType::Restart
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RollAgainMsg {
    #[serde(rename = "type")]
    pub type_: RollAgainMsgType,
}
impl From<&RollAgainMsg> for RollAgainMsg {
    fn from(value: &RollAgainMsg) -> Self {
        value.clone()
    }
}
impl RollAgainMsg {
    pub fn builder() -> builder::RollAgainMsg {
        builder::RollAgainMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RollAgainMsgType {
    #[serde(rename = "roll_again")]
    RollAgain,
}
impl From<&RollAgainMsgType> for RollAgainMsgType {
    fn from(value: &RollAgainMsgType) -> Self {
        value.clone()
    }
}
impl ToString for RollAgainMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::RollAgain => "roll_again".to_string(),
        }
    }
}
impl std::str::FromStr for RollAgainMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "roll_again" => Ok(Self::RollAgain),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RollAgainMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RollAgainMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RollAgainMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RollAgainMsgType {
    fn default() -> Self {
        RollAgainMsgType::RollAgain
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RollMsg {
    pub rolls: Vec<i64>,
    #[serde(rename = "type")]
    pub type_: RollMsgType,
}
impl From<&RollMsg> for RollMsg {
    fn from(value: &RollMsg) -> Self {
        value.clone()
    }
}
impl RollMsg {
    pub fn builder() -> builder::RollMsg {
        builder::RollMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RollMsgType {
    #[serde(rename = "roll")]
    Roll,
}
impl From<&RollMsgType> for RollMsgType {
    fn from(value: &RollMsgType) -> Self {
        value.clone()
    }
}
impl ToString for RollMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Roll => "roll".to_string(),
        }
    }
}
impl std::str::FromStr for RollMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "roll" => Ok(Self::Roll),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RollMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RollMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RollMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RollMsgType {
    fn default() -> Self {
        RollMsgType::Roll
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Room {
    pub code: String,
    pub host_name: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub player_count: i64,
}
impl From<&Room> for Room {
    fn from(value: &Room) -> Self {
        value.clone()
    }
}
impl Room {
    pub fn builder() -> builder::Room {
        builder::Room::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RoomListMsg {
    pub rooms: Vec<Room>,
    #[serde(rename = "type", default = "defaults::room_list_msg_type")]
    pub type_: RoomListMsgType,
}
impl From<&RoomListMsg> for RoomListMsg {
    fn from(value: &RoomListMsg) -> Self {
        value.clone()
    }
}
impl RoomListMsg {
    pub fn builder() -> builder::RoomListMsg {
        builder::RoomListMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RoomListMsgType {
    #[serde(rename = "room_list")]
    RoomList,
}
impl From<&RoomListMsgType> for RoomListMsgType {
    fn from(value: &RoomListMsgType) -> Self {
        value.clone()
    }
}
impl ToString for RoomListMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::RoomList => "room_list".to_string(),
        }
    }
}
impl std::str::FromStr for RoomListMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "room_list" => Ok(Self::RoomList),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RoomListMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RoomListMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RoomListMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RoomListMsgType {
    fn default() -> Self {
        RoomListMsgType::RoomList
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerMsg {
    ErrorMsg(GameError),
    RoomListMsg(RoomListMsg),
    RedirectMsg(Redirect),
    RefetchPlayerMsg(RefetchPlayerMsg),
    WelcomeMsg(WelcomeMsg),
    RestartMsg(RestartMsg),
    SpectatorsMsg(SpectatorsMsg),
    WinMsg(WinMsg),
    RollMsg(RollMsg),
    RollAgainMsg(RollAgainMsg),
    JoinMsg(JoinMsg),
    DisconnectMsg(DisconnectMsg),
    ReconnectMsg(ReconnectMsg),
    KickMsg(KickMsg),
    ChatMsg(ChatMsg),
    UpdateTurnMsg(UpdateTurnMsg),
    UpdateNameMsg(UpdateNameMsg),
    UpdateMsg(UpdateMsg),
}
impl From<&ServerMsg> for ServerMsg {
    fn from(value: &ServerMsg) -> Self {
        value.clone()
    }
}
impl From<RoomListMsg> for ServerMsg {
    fn from(value: RoomListMsg) -> Self {
        Self::RoomListMsg(value)
    }
}
impl From<RefetchPlayerMsg> for ServerMsg {
    fn from(value: RefetchPlayerMsg) -> Self {
        Self::RefetchPlayerMsg(value)
    }
}
impl From<WelcomeMsg> for ServerMsg {
    fn from(value: WelcomeMsg) -> Self {
        Self::WelcomeMsg(value)
    }
}
impl From<RestartMsg> for ServerMsg {
    fn from(value: RestartMsg) -> Self {
        Self::RestartMsg(value)
    }
}
impl From<SpectatorsMsg> for ServerMsg {
    fn from(value: SpectatorsMsg) -> Self {
        Self::SpectatorsMsg(value)
    }
}
impl From<WinMsg> for ServerMsg {
    fn from(value: WinMsg) -> Self {
        Self::WinMsg(value)
    }
}
impl From<RollMsg> for ServerMsg {
    fn from(value: RollMsg) -> Self {
        Self::RollMsg(value)
    }
}
impl From<RollAgainMsg> for ServerMsg {
    fn from(value: RollAgainMsg) -> Self {
        Self::RollAgainMsg(value)
    }
}
impl From<JoinMsg> for ServerMsg {
    fn from(value: JoinMsg) -> Self {
        Self::JoinMsg(value)
    }
}
impl From<DisconnectMsg> for ServerMsg {
    fn from(value: DisconnectMsg) -> Self {
        Self::DisconnectMsg(value)
    }
}
impl From<ReconnectMsg> for ServerMsg {
    fn from(value: ReconnectMsg) -> Self {
        Self::ReconnectMsg(value)
    }
}
impl From<KickMsg> for ServerMsg {
    fn from(value: KickMsg) -> Self {
        Self::KickMsg(value)
    }
}
impl From<ChatMsg> for ServerMsg {
    fn from(value: ChatMsg) -> Self {
        Self::ChatMsg(value)
    }
}
impl From<UpdateTurnMsg> for ServerMsg {
    fn from(value: UpdateTurnMsg) -> Self {
        Self::UpdateTurnMsg(value)
    }
}
impl From<UpdateNameMsg> for ServerMsg {
    fn from(value: UpdateNameMsg) -> Self {
        Self::UpdateNameMsg(value)
    }
}
impl From<UpdateMsg> for ServerMsg {
    fn from(value: UpdateMsg) -> Self {
        Self::UpdateMsg(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerPlayer {
    pub connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crowned: Option<bool>,
    pub doubles_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub roll_count: i64,
    pub score: i64,
    pub session: String,
    pub skip_count: i64,
    pub turn_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    pub win_count: i64,
}
impl From<&ServerPlayer> for ServerPlayer {
    fn from(value: &ServerPlayer) -> Self {
        value.clone()
    }
}
impl ServerPlayer {
    pub fn builder() -> builder::ServerPlayer {
        builder::ServerPlayer::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpectatorsMsg {
    pub count: i64,
    #[serde(rename = "type")]
    pub type_: SpectatorsMsgType,
}
impl From<&SpectatorsMsg> for SpectatorsMsg {
    fn from(value: &SpectatorsMsg) -> Self {
        value.clone()
    }
}
impl SpectatorsMsg {
    pub fn builder() -> builder::SpectatorsMsg {
        builder::SpectatorsMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SpectatorsMsgType {
    #[serde(rename = "spectators")]
    Spectators,
}
impl From<&SpectatorsMsgType> for SpectatorsMsgType {
    fn from(value: &SpectatorsMsgType) -> Self {
        value.clone()
    }
}
impl ToString for SpectatorsMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Spectators => "spectators".to_string(),
        }
    }
}
impl std::str::FromStr for SpectatorsMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "spectators" => Ok(Self::Spectators),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SpectatorsMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SpectatorsMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SpectatorsMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for SpectatorsMsgType {
    fn default() -> Self {
        SpectatorsMsgType::Spectators
    }
}
///TODO: add descriptions to these things
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateMsg {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset: Option<bool>,
    pub score: i64,
    #[serde(rename = "type")]
    pub type_: UpdateMsgType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub used: Vec<bool>,
}
impl From<&UpdateMsg> for UpdateMsg {
    fn from(value: &UpdateMsg) -> Self {
        value.clone()
    }
}
impl UpdateMsg {
    pub fn builder() -> builder::UpdateMsg {
        builder::UpdateMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UpdateMsgType {
    #[serde(rename = "update")]
    Update,
}
impl From<&UpdateMsgType> for UpdateMsgType {
    fn from(value: &UpdateMsgType) -> Self {
        value.clone()
    }
}
impl ToString for UpdateMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Update => "update".to_string(),
        }
    }
}
impl std::str::FromStr for UpdateMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "update" => Ok(Self::Update),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for UpdateMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UpdateMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UpdateMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for UpdateMsgType {
    fn default() -> Self {
        UpdateMsgType::Update
    }
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateNameMsg {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: UpdateNameMsgType,
}
impl From<&UpdateNameMsg> for UpdateNameMsg {
    fn from(value: &UpdateNameMsg) -> Self {
        value.clone()
    }
}
impl UpdateNameMsg {
    pub fn builder() -> builder::UpdateNameMsg {
        builder::UpdateNameMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UpdateNameMsgType {
    #[serde(rename = "update_name")]
    UpdateName,
}
impl From<&UpdateNameMsgType> for UpdateNameMsgType {
    fn from(value: &UpdateNameMsgType) -> Self {
        value.clone()
    }
}
impl ToString for UpdateNameMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::UpdateName => "update_name".to_string(),
        }
    }
}
impl std::str::FromStr for UpdateNameMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "update_name" => Ok(Self::UpdateName),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for UpdateNameMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UpdateNameMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UpdateNameMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for UpdateNameMsgType {
    fn default() -> Self {
        UpdateNameMsgType::UpdateName
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateTurnMsg {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    #[serde(rename = "type")]
    pub type_: UpdateTurnMsgType,
}
impl From<&UpdateTurnMsg> for UpdateTurnMsg {
    fn from(value: &UpdateTurnMsg) -> Self {
        value.clone()
    }
}
impl UpdateTurnMsg {
    pub fn builder() -> builder::UpdateTurnMsg {
        builder::UpdateTurnMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum UpdateTurnMsgType {
    #[serde(rename = "update_turn")]
    UpdateTurn,
}
impl From<&UpdateTurnMsgType> for UpdateTurnMsgType {
    fn from(value: &UpdateTurnMsgType) -> Self {
        value.clone()
    }
}
impl ToString for UpdateTurnMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::UpdateTurn => "update_turn".to_string(),
        }
    }
}
impl std::str::FromStr for UpdateTurnMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "update_turn" => Ok(Self::UpdateTurn),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for UpdateTurnMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UpdateTurnMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UpdateTurnMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for UpdateTurnMsgType {
    fn default() -> Self {
        UpdateTurnMsgType::UpdateTurn
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub achievements: Option<Vec<Achievement>>,
    pub color: Color,
    #[serde(rename = "createdDate")]
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub dice: Dice,
    pub donor: bool,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pubkey_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<UserStats>,
    pub username: String,
}
impl From<&UserData> for UserData {
    fn from(value: &UserData) -> Self {
        value.clone()
    }
}
impl UserData {
    pub fn builder() -> builder::UserData {
        builder::UserData::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum UserId {
    User(uuid::Uuid),
    Anonymous(uuid::Uuid),
}
impl From<&UserId> for UserId {
    fn from(value: &UserId) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserStats {
    pub doubles: i64,
    pub games: i64,
    pub rolls: i64,
    pub wins: i64,
}
impl From<&UserStats> for UserStats {
    fn from(value: &UserStats) -> Self {
        value.clone()
    }
}
impl UserStats {
    pub fn builder() -> builder::UserStats {
        builder::UserStats::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WelcomeMsg {
    #[serde(rename = "chatLog")]
    pub chat_log: Vec<String>,
    pub id: i64,
    pub players: Vec<Player>,
    #[serde(rename = "privateSession")]
    pub private_session: bool,
    pub rolled: bool,
    pub rolls: Vec<i64>,
    pub spectators: i64,
    pub turn_index: i64,
    #[serde(rename = "type")]
    pub type_: WelcomeMsgType,
    pub used: Vec<bool>,
    pub victory: bool,
}
impl From<&WelcomeMsg> for WelcomeMsg {
    fn from(value: &WelcomeMsg) -> Self {
        value.clone()
    }
}
impl WelcomeMsg {
    pub fn builder() -> builder::WelcomeMsg {
        builder::WelcomeMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WelcomeMsgType {
    #[serde(rename = "welcome")]
    Welcome,
}
impl From<&WelcomeMsgType> for WelcomeMsgType {
    fn from(value: &WelcomeMsgType) -> Self {
        value.clone()
    }
}
impl ToString for WelcomeMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Welcome => "welcome".to_string(),
        }
    }
}
impl std::str::FromStr for WelcomeMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "welcome" => Ok(Self::Welcome),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for WelcomeMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WelcomeMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WelcomeMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for WelcomeMsgType {
    fn default() -> Self {
        WelcomeMsgType::Welcome
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WinMsg {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: WinMsgType,
}
impl From<&WinMsg> for WinMsg {
    fn from(value: &WinMsg) -> Self {
        value.clone()
    }
}
impl WinMsg {
    pub fn builder() -> builder::WinMsg {
        builder::WinMsg::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WinMsgType {
    #[serde(rename = "win")]
    Win,
}
impl From<&WinMsgType> for WinMsgType {
    fn from(value: &WinMsgType) -> Self {
        value.clone()
    }
}
impl ToString for WinMsgType {
    fn to_string(&self) -> String {
        match *self {
            Self::Win => "win".to_string(),
        }
    }
}
impl std::str::FromStr for WinMsgType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "win" => Ok(Self::Win),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for WinMsgType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WinMsgType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WinMsgType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for WinMsgType {
    fn default() -> Self {
        WinMsgType::Win
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Achievement {
        id: Result<String, String>,
        progress: Result<i64, String>,
        rd: Result<Option<i64>, String>,
        rn: Result<Option<i64>, String>,
        unlocked: Result<String, String>,
    }
    impl Default for Achievement {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                progress: Err("no value supplied for progress".to_string()),
                rd: Err("no value supplied for rd".to_string()),
                rn: Err("no value supplied for rn".to_string()),
                unlocked: Err("no value supplied for unlocked".to_string()),
            }
        }
    }
    impl Achievement {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn progress<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress: {}", e));
            self
        }
        pub fn rd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rd: {}", e));
            self
        }
        pub fn rn<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rn: {}", e));
            self
        }
        pub fn unlocked<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.unlocked = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unlocked: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Achievement> for super::Achievement {
        type Error = String;
        fn try_from(value: Achievement) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                progress: value.progress?,
                rd: value.rd?,
                rn: value.rn?,
                unlocked: value.unlocked?,
            })
        }
    }
    impl From<super::Achievement> for Achievement {
        fn from(value: super::Achievement) -> Self {
            Self {
                id: Ok(value.id),
                progress: Ok(value.progress),
                rd: Ok(value.rd),
                rn: Ok(value.rn),
                unlocked: Ok(value.unlocked),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AchievementData {
        description: Result<String, String>,
        id: Result<String, String>,
        image_url: Result<Option<String>, String>,
        max_progress: Result<i64, String>,
        name: Result<String, String>,
    }
    impl Default for AchievementData {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                image_url: Err("no value supplied for image_url".to_string()),
                max_progress: Err("no value supplied for max_progress".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl AchievementData {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {}", e));
            self
        }
        pub fn max_progress<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max_progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_progress: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementData> for super::AchievementData {
        type Error = String;
        fn try_from(value: AchievementData) -> Result<Self, String> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                image_url: value.image_url?,
                max_progress: value.max_progress?,
                name: value.name?,
            })
        }
    }
    impl From<super::AchievementData> for AchievementData {
        fn from(value: super::AchievementData) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                image_url: Ok(value.image_url),
                max_progress: Ok(value.max_progress),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AchievementProgress {
        achievement_id: Result<String, String>,
        progress: Result<i64, String>,
        type_: Result<super::AchievementProgressType, String>,
        user_id: Result<super::UserId, String>,
        user_index: Result<i64, String>,
    }
    impl Default for AchievementProgress {
        fn default() -> Self {
            Self {
                achievement_id: Err("no value supplied for achievement_id".to_string()),
                progress: Err("no value supplied for progress".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
                user_index: Err("no value supplied for user_index".to_string()),
            }
        }
    }
    impl AchievementProgress {
        pub fn achievement_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.achievement_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for achievement_id: {}", e));
            self
        }
        pub fn progress<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for progress: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementProgressType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserId>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn user_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.user_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_index: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementProgress> for super::AchievementProgress {
        type Error = String;
        fn try_from(value: AchievementProgress) -> Result<Self, String> {
            Ok(Self {
                achievement_id: value.achievement_id?,
                progress: value.progress?,
                type_: value.type_?,
                user_id: value.user_id?,
                user_index: value.user_index?,
            })
        }
    }
    impl From<super::AchievementProgress> for AchievementProgress {
        fn from(value: super::AchievementProgress) -> Self {
            Self {
                achievement_id: Ok(value.achievement_id),
                progress: Ok(value.progress),
                type_: Ok(value.type_),
                user_id: Ok(value.user_id),
                user_index: Ok(value.user_index),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AchievementUnlock {
        description: Result<String, String>,
        id: Result<String, String>,
        image_url: Result<Option<String>, String>,
        max_progress: Result<i64, String>,
        name: Result<String, String>,
        type_: Result<super::AchievementUnlockType, String>,
        user_id: Result<uuid::Uuid, String>,
        user_index: Result<i64, String>,
    }
    impl Default for AchievementUnlock {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                image_url: Err("no value supplied for image_url".to_string()),
                max_progress: Err("no value supplied for max_progress".to_string()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
                user_index: Err("no value supplied for user_index".to_string()),
            }
        }
    }
    impl AchievementUnlock {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {}", e));
            self
        }
        pub fn max_progress<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max_progress = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_progress: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AchievementUnlockType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<uuid::Uuid>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn user_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.user_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_index: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AchievementUnlock> for super::AchievementUnlock {
        type Error = String;
        fn try_from(value: AchievementUnlock) -> Result<Self, String> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                image_url: value.image_url?,
                max_progress: value.max_progress?,
                name: value.name?,
                type_: value.type_?,
                user_id: value.user_id?,
                user_index: value.user_index?,
            })
        }
    }
    impl From<super::AchievementUnlock> for AchievementUnlock {
        fn from(value: super::AchievementUnlock) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                image_url: Ok(value.image_url),
                max_progress: Ok(value.max_progress),
                name: Ok(value.name),
                type_: Ok(value.type_),
                user_id: Ok(value.user_id),
                user_index: Ok(value.user_index),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuthDonateResponse {
        link: Result<String, String>,
    }
    impl Default for AuthDonateResponse {
        fn default() -> Self {
            Self {
                link: Err("no value supplied for link".to_string()),
            }
        }
    }
    impl AuthDonateResponse {
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AuthDonateResponse> for super::AuthDonateResponse {
        type Error = String;
        fn try_from(value: AuthDonateResponse) -> Result<Self, String> {
            Ok(Self { link: value.link? })
        }
    }
    impl From<super::AuthDonateResponse> for AuthDonateResponse {
        fn from(value: super::AuthDonateResponse) -> Self {
            Self {
                link: Ok(value.link),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuthRefreshTokenResponse {
        access_token: Result<String, String>,
    }
    impl Default for AuthRefreshTokenResponse {
        fn default() -> Self {
            Self {
                access_token: Err("no value supplied for access_token".to_string()),
            }
        }
    }
    impl AuthRefreshTokenResponse {
        pub fn access_token<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.access_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access_token: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AuthRefreshTokenResponse> for super::AuthRefreshTokenResponse {
        type Error = String;
        fn try_from(value: AuthRefreshTokenResponse) -> Result<Self, String> {
            Ok(Self {
                access_token: value.access_token?,
            })
        }
    }
    impl From<super::AuthRefreshTokenResponse> for AuthRefreshTokenResponse {
        fn from(value: super::AuthRefreshTokenResponse) -> Self {
            Self {
                access_token: Ok(value.access_token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ChatMsg {
        msg: Result<String, String>,
        type_: Result<super::ChatMsgType, String>,
    }
    impl Default for ChatMsg {
        fn default() -> Self {
            Self {
                msg: Err("no value supplied for msg".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ChatMsg {
        pub fn msg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.msg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for msg: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChatMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ChatMsg> for super::ChatMsg {
        type Error = String;
        fn try_from(value: ChatMsg) -> Result<Self, String> {
            Ok(Self {
                msg: value.msg?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ChatMsg> for ChatMsg {
        fn from(value: super::ChatMsg) -> Self {
            Self {
                msg: Ok(value.msg),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Color {
        hue: Result<f64, String>,
        sat: Result<f64, String>,
    }
    impl Default for Color {
        fn default() -> Self {
            Self {
                hue: Err("no value supplied for hue".to_string()),
                sat: Err("no value supplied for sat".to_string()),
            }
        }
    }
    impl Color {
        pub fn hue<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.hue = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hue: {}", e));
            self
        }
        pub fn sat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.sat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sat: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Color> for super::Color {
        type Error = String;
        fn try_from(value: Color) -> Result<Self, String> {
            Ok(Self {
                hue: value.hue?,
                sat: value.sat?,
            })
        }
    }
    impl From<super::Color> for Color {
        fn from(value: super::Color) -> Self {
            Self {
                hue: Ok(value.hue),
                sat: Ok(value.sat),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Dice {
        type_: Result<super::DiceType, String>,
    }
    impl Default for Dice {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Dice {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DiceType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Dice> for super::Dice {
        type Error = String;
        fn try_from(value: Dice) -> Result<Self, String> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl From<super::Dice> for Dice {
        fn from(value: super::Dice) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DieRoll {
        used: Result<bool, String>,
        value: Result<i64, String>,
    }
    impl Default for DieRoll {
        fn default() -> Self {
            Self {
                used: Err("no value supplied for used".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl DieRoll {
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DieRoll> for super::DieRoll {
        type Error = String;
        fn try_from(value: DieRoll) -> Result<Self, String> {
            Ok(Self {
                used: value.used?,
                value: value.value?,
            })
        }
    }
    impl From<super::DieRoll> for DieRoll {
        fn from(value: super::DieRoll) -> Self {
            Self {
                used: Ok(value.used),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DisconnectMsg {
        id: Result<i64, String>,
        type_: Result<super::DisconnectMsgType, String>,
    }
    impl Default for DisconnectMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DisconnectMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DisconnectMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DisconnectMsg> for super::DisconnectMsg {
        type Error = String;
        fn try_from(value: DisconnectMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DisconnectMsg> for DisconnectMsg {
        fn from(value: super::DisconnectMsg) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GameError {
        error: Result<String, String>,
        type_: Result<super::GameErrorType, String>,
    }
    impl Default for GameError {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GameError {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GameErrorType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GameError> for super::GameError {
        type Error = String;
        fn try_from(value: GameError) -> Result<Self, String> {
            Ok(Self {
                error: value.error?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GameError> for GameError {
        fn from(value: super::GameError) -> Self {
            Self {
                error: Ok(value.error),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GameState {
        chat_log: Result<Vec<String>, String>,
        players: Result<Vec<super::ServerPlayer>, String>,
        private_session: Result<bool, String>,
        rolled: Result<bool, String>,
        rolls: Result<Vec<i64>, String>,
        spectators: Result<i64, String>,
        turn_index: Result<i64, String>,
        type_: Result<super::GameStateType, String>,
        used: Result<Vec<bool>, String>,
        victory: Result<bool, String>,
    }
    impl Default for GameState {
        fn default() -> Self {
            Self {
                chat_log: Err("no value supplied for chat_log".to_string()),
                players: Err("no value supplied for players".to_string()),
                private_session: Err("no value supplied for private_session".to_string()),
                rolled: Err("no value supplied for rolled".to_string()),
                rolls: Err("no value supplied for rolls".to_string()),
                spectators: Err("no value supplied for spectators".to_string()),
                turn_index: Err("no value supplied for turn_index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                used: Err("no value supplied for used".to_string()),
                victory: Err("no value supplied for victory".to_string()),
            }
        }
    }
    impl GameState {
        pub fn chat_log<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.chat_log = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chat_log: {}", e));
            self
        }
        pub fn players<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ServerPlayer>>,
            T::Error: std::fmt::Display,
        {
            self.players = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for players: {}", e));
            self
        }
        pub fn private_session<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.private_session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for private_session: {}", e));
            self
        }
        pub fn rolled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rolled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolled: {}", e));
            self
        }
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn spectators<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.spectators = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spectators: {}", e));
            self
        }
        pub fn turn_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.turn_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for turn_index: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GameStateType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<bool>>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
        pub fn victory<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.victory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for victory: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GameState> for super::GameState {
        type Error = String;
        fn try_from(value: GameState) -> Result<Self, String> {
            Ok(Self {
                chat_log: value.chat_log?,
                players: value.players?,
                private_session: value.private_session?,
                rolled: value.rolled?,
                rolls: value.rolls?,
                spectators: value.spectators?,
                turn_index: value.turn_index?,
                type_: value.type_?,
                used: value.used?,
                victory: value.victory?,
            })
        }
    }
    impl From<super::GameState> for GameState {
        fn from(value: super::GameState) -> Self {
            Self {
                chat_log: Ok(value.chat_log),
                players: Ok(value.players),
                private_session: Ok(value.private_session),
                rolled: Ok(value.rolled),
                rolls: Ok(value.rolls),
                spectators: Ok(value.spectators),
                turn_index: Ok(value.turn_index),
                type_: Ok(value.type_),
                used: Ok(value.used),
                victory: Ok(value.victory),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IGameState {
        chat_log: Result<Vec<String>, String>,
        players: Result<Vec<super::Player>, String>,
        private_session: Result<bool, String>,
        rolled: Result<bool, String>,
        rolls: Result<Vec<i64>, String>,
        spectators: Result<i64, String>,
        turn_index: Result<i64, String>,
        used: Result<Vec<bool>, String>,
        victory: Result<bool, String>,
    }
    impl Default for IGameState {
        fn default() -> Self {
            Self {
                chat_log: Err("no value supplied for chat_log".to_string()),
                players: Err("no value supplied for players".to_string()),
                private_session: Err("no value supplied for private_session".to_string()),
                rolled: Err("no value supplied for rolled".to_string()),
                rolls: Err("no value supplied for rolls".to_string()),
                spectators: Err("no value supplied for spectators".to_string()),
                turn_index: Err("no value supplied for turn_index".to_string()),
                used: Err("no value supplied for used".to_string()),
                victory: Err("no value supplied for victory".to_string()),
            }
        }
    }
    impl IGameState {
        pub fn chat_log<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.chat_log = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chat_log: {}", e));
            self
        }
        pub fn players<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Player>>,
            T::Error: std::fmt::Display,
        {
            self.players = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for players: {}", e));
            self
        }
        pub fn private_session<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.private_session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for private_session: {}", e));
            self
        }
        pub fn rolled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rolled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolled: {}", e));
            self
        }
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn spectators<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.spectators = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spectators: {}", e));
            self
        }
        pub fn turn_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.turn_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for turn_index: {}", e));
            self
        }
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<bool>>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
        pub fn victory<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.victory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for victory: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IGameState> for super::IGameState {
        type Error = String;
        fn try_from(value: IGameState) -> Result<Self, String> {
            Ok(Self {
                chat_log: value.chat_log?,
                players: value.players?,
                private_session: value.private_session?,
                rolled: value.rolled?,
                rolls: value.rolls?,
                spectators: value.spectators?,
                turn_index: value.turn_index?,
                used: value.used?,
                victory: value.victory?,
            })
        }
    }
    impl From<super::IGameState> for IGameState {
        fn from(value: super::IGameState) -> Self {
            Self {
                chat_log: Ok(value.chat_log),
                players: Ok(value.players),
                private_session: Ok(value.private_session),
                rolled: Ok(value.rolled),
                rolls: Ok(value.rolls),
                spectators: Ok(value.spectators),
                turn_index: Ok(value.turn_index),
                used: Ok(value.used),
                victory: Ok(value.victory),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JoinMsg {
        id: Result<i64, String>,
        name: Result<Option<String>, String>,
        type_: Result<super::JoinMsgType, String>,
        user_id: Result<Option<String>, String>,
    }
    impl Default for JoinMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl JoinMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::JoinMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<JoinMsg> for super::JoinMsg {
        type Error = String;
        fn try_from(value: JoinMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                type_: value.type_?,
                user_id: value.user_id?,
            })
        }
    }
    impl From<super::JoinMsg> for JoinMsg {
        fn from(value: super::JoinMsg) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                type_: Ok(value.type_),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KickMsg {
        id: Result<i64, String>,
        type_: Result<super::KickMsgType, String>,
    }
    impl Default for KickMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl KickMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::KickMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<KickMsg> for super::KickMsg {
        type Error = String;
        fn try_from(value: KickMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::KickMsg> for KickMsg {
        fn from(value: super::KickMsg) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Player {
        connected: Result<bool, String>,
        crowned: Result<Option<bool>, String>,
        name: Result<Option<String>, String>,
        score: Result<i64, String>,
        skip_count: Result<i64, String>,
        user_id: Result<Option<uuid::Uuid>, String>,
        win_count: Result<i64, String>,
    }
    impl Default for Player {
        fn default() -> Self {
            Self {
                connected: Err("no value supplied for connected".to_string()),
                crowned: Ok(Default::default()),
                name: Ok(Default::default()),
                score: Err("no value supplied for score".to_string()),
                skip_count: Err("no value supplied for skip_count".to_string()),
                user_id: Ok(Default::default()),
                win_count: Err("no value supplied for win_count".to_string()),
            }
        }
    }
    impl Player {
        pub fn connected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.connected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connected: {}", e));
            self
        }
        pub fn crowned<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.crowned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for crowned: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn skip_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.skip_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skip_count: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<uuid::Uuid>>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn win_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.win_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for win_count: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Player> for super::Player {
        type Error = String;
        fn try_from(value: Player) -> Result<Self, String> {
            Ok(Self {
                connected: value.connected?,
                crowned: value.crowned?,
                name: value.name?,
                score: value.score?,
                skip_count: value.skip_count?,
                user_id: value.user_id?,
                win_count: value.win_count?,
            })
        }
    }
    impl From<super::Player> for Player {
        fn from(value: super::Player) -> Self {
            Self {
                connected: Ok(value.connected),
                crowned: Ok(value.crowned),
                name: Ok(value.name),
                score: Ok(value.score),
                skip_count: Ok(value.skip_count),
                user_id: Ok(value.user_id),
                win_count: Ok(value.win_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReconnectMsg {
        id: Result<i64, String>,
        name: Result<Option<String>, String>,
        type_: Result<super::ReconnectMsgType, String>,
        user_id: Result<Option<String>, String>,
    }
    impl Default for ReconnectMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                user_id: Ok(Default::default()),
            }
        }
    }
    impl ReconnectMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ReconnectMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReconnectMsg> for super::ReconnectMsg {
        type Error = String;
        fn try_from(value: ReconnectMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                type_: value.type_?,
                user_id: value.user_id?,
            })
        }
    }
    impl From<super::ReconnectMsg> for ReconnectMsg {
        fn from(value: super::ReconnectMsg) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                type_: Ok(value.type_),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Redirect {
        room: Result<String, String>,
        type_: Result<super::RedirectType, String>,
    }
    impl Default for Redirect {
        fn default() -> Self {
            Self {
                room: Err("no value supplied for room".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Redirect {
        pub fn room<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.room = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RedirectType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Redirect> for super::Redirect {
        type Error = String;
        fn try_from(value: Redirect) -> Result<Self, String> {
            Ok(Self {
                room: value.room?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Redirect> for Redirect {
        fn from(value: super::Redirect) -> Self {
            Self {
                room: Ok(value.room),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RefetchPlayerMsg {
        type_: Result<super::RefetchPlayerMsgType, String>,
        user_id: Result<String, String>,
    }
    impl Default for RefetchPlayerMsg {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl RefetchPlayerMsg {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RefetchPlayerMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RefetchPlayerMsg> for super::RefetchPlayerMsg {
        type Error = String;
        fn try_from(value: RefetchPlayerMsg) -> Result<Self, String> {
            Ok(Self {
                type_: value.type_?,
                user_id: value.user_id?,
            })
        }
    }
    impl From<super::RefetchPlayerMsg> for RefetchPlayerMsg {
        fn from(value: super::RefetchPlayerMsg) -> Self {
            Self {
                type_: Ok(value.type_),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReportStats {
        doubles: Result<i64, String>,
        games: Result<i64, String>,
        rolls: Result<i64, String>,
        user_id: Result<super::UserId, String>,
        wins: Result<i64, String>,
    }
    impl Default for ReportStats {
        fn default() -> Self {
            Self {
                doubles: Err("no value supplied for doubles".to_string()),
                games: Err("no value supplied for games".to_string()),
                rolls: Err("no value supplied for rolls".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
                wins: Err("no value supplied for wins".to_string()),
            }
        }
    }
    impl ReportStats {
        pub fn doubles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.doubles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for doubles: {}", e));
            self
        }
        pub fn games<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.games = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for games: {}", e));
            self
        }
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UserId>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn wins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.wins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wins: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReportStats> for super::ReportStats {
        type Error = String;
        fn try_from(value: ReportStats) -> Result<Self, String> {
            Ok(Self {
                doubles: value.doubles?,
                games: value.games?,
                rolls: value.rolls?,
                user_id: value.user_id?,
                wins: value.wins?,
            })
        }
    }
    impl From<super::ReportStats> for ReportStats {
        fn from(value: super::ReportStats) -> Self {
            Self {
                doubles: Ok(value.doubles),
                games: Ok(value.games),
                rolls: Ok(value.rolls),
                user_id: Ok(value.user_id),
                wins: Ok(value.wins),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RestartMsg {
        id: Result<i64, String>,
        type_: Result<super::RestartMsgType, String>,
    }
    impl Default for RestartMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RestartMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RestartMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RestartMsg> for super::RestartMsg {
        type Error = String;
        fn try_from(value: RestartMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::RestartMsg> for RestartMsg {
        fn from(value: super::RestartMsg) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RollAgainMsg {
        type_: Result<super::RollAgainMsgType, String>,
    }
    impl Default for RollAgainMsg {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RollAgainMsg {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RollAgainMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RollAgainMsg> for super::RollAgainMsg {
        type Error = String;
        fn try_from(value: RollAgainMsg) -> Result<Self, String> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl From<super::RollAgainMsg> for RollAgainMsg {
        fn from(value: super::RollAgainMsg) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RollMsg {
        rolls: Result<Vec<i64>, String>,
        type_: Result<super::RollMsgType, String>,
    }
    impl Default for RollMsg {
        fn default() -> Self {
            Self {
                rolls: Err("no value supplied for rolls".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RollMsg {
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RollMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RollMsg> for super::RollMsg {
        type Error = String;
        fn try_from(value: RollMsg) -> Result<Self, String> {
            Ok(Self {
                rolls: value.rolls?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::RollMsg> for RollMsg {
        fn from(value: super::RollMsg) -> Self {
            Self {
                rolls: Ok(value.rolls),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Room {
        code: Result<String, String>,
        host_name: Result<String, String>,
        last_updated: Result<chrono::DateTime<chrono::Utc>, String>,
        player_count: Result<i64, String>,
    }
    impl Default for Room {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                host_name: Err("no value supplied for host_name".to_string()),
                last_updated: Err("no value supplied for last_updated".to_string()),
                player_count: Err("no value supplied for player_count".to_string()),
            }
        }
    }
    impl Room {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn host_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.host_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host_name: {}", e));
            self
        }
        pub fn last_updated<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.last_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_updated: {}", e));
            self
        }
        pub fn player_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.player_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for player_count: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Room> for super::Room {
        type Error = String;
        fn try_from(value: Room) -> Result<Self, String> {
            Ok(Self {
                code: value.code?,
                host_name: value.host_name?,
                last_updated: value.last_updated?,
                player_count: value.player_count?,
            })
        }
    }
    impl From<super::Room> for Room {
        fn from(value: super::Room) -> Self {
            Self {
                code: Ok(value.code),
                host_name: Ok(value.host_name),
                last_updated: Ok(value.last_updated),
                player_count: Ok(value.player_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RoomListMsg {
        rooms: Result<Vec<super::Room>, String>,
        type_: Result<super::RoomListMsgType, String>,
    }
    impl Default for RoomListMsg {
        fn default() -> Self {
            Self {
                rooms: Err("no value supplied for rooms".to_string()),
                type_: Ok(super::defaults::room_list_msg_type()),
            }
        }
    }
    impl RoomListMsg {
        pub fn rooms<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Room>>,
            T::Error: std::fmt::Display,
        {
            self.rooms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rooms: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RoomListMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RoomListMsg> for super::RoomListMsg {
        type Error = String;
        fn try_from(value: RoomListMsg) -> Result<Self, String> {
            Ok(Self {
                rooms: value.rooms?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::RoomListMsg> for RoomListMsg {
        fn from(value: super::RoomListMsg) -> Self {
            Self {
                rooms: Ok(value.rooms),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ServerPlayer {
        connected: Result<bool, String>,
        crowned: Result<Option<bool>, String>,
        doubles_count: Result<i64, String>,
        name: Result<Option<String>, String>,
        roll_count: Result<i64, String>,
        score: Result<i64, String>,
        session: Result<String, String>,
        skip_count: Result<i64, String>,
        turn_count: Result<i64, String>,
        user_id: Result<Option<uuid::Uuid>, String>,
        win_count: Result<i64, String>,
    }
    impl Default for ServerPlayer {
        fn default() -> Self {
            Self {
                connected: Err("no value supplied for connected".to_string()),
                crowned: Ok(Default::default()),
                doubles_count: Err("no value supplied for doubles_count".to_string()),
                name: Ok(Default::default()),
                roll_count: Err("no value supplied for roll_count".to_string()),
                score: Err("no value supplied for score".to_string()),
                session: Err("no value supplied for session".to_string()),
                skip_count: Err("no value supplied for skip_count".to_string()),
                turn_count: Err("no value supplied for turn_count".to_string()),
                user_id: Ok(Default::default()),
                win_count: Err("no value supplied for win_count".to_string()),
            }
        }
    }
    impl ServerPlayer {
        pub fn connected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.connected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for connected: {}", e));
            self
        }
        pub fn crowned<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.crowned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for crowned: {}", e));
            self
        }
        pub fn doubles_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.doubles_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for doubles_count: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn roll_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.roll_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for roll_count: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn session<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session: {}", e));
            self
        }
        pub fn skip_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.skip_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skip_count: {}", e));
            self
        }
        pub fn turn_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.turn_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for turn_count: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<uuid::Uuid>>,
            T::Error: std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
        pub fn win_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.win_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for win_count: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ServerPlayer> for super::ServerPlayer {
        type Error = String;
        fn try_from(value: ServerPlayer) -> Result<Self, String> {
            Ok(Self {
                connected: value.connected?,
                crowned: value.crowned?,
                doubles_count: value.doubles_count?,
                name: value.name?,
                roll_count: value.roll_count?,
                score: value.score?,
                session: value.session?,
                skip_count: value.skip_count?,
                turn_count: value.turn_count?,
                user_id: value.user_id?,
                win_count: value.win_count?,
            })
        }
    }
    impl From<super::ServerPlayer> for ServerPlayer {
        fn from(value: super::ServerPlayer) -> Self {
            Self {
                connected: Ok(value.connected),
                crowned: Ok(value.crowned),
                doubles_count: Ok(value.doubles_count),
                name: Ok(value.name),
                roll_count: Ok(value.roll_count),
                score: Ok(value.score),
                session: Ok(value.session),
                skip_count: Ok(value.skip_count),
                turn_count: Ok(value.turn_count),
                user_id: Ok(value.user_id),
                win_count: Ok(value.win_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpectatorsMsg {
        count: Result<i64, String>,
        type_: Result<super::SpectatorsMsgType, String>,
    }
    impl Default for SpectatorsMsg {
        fn default() -> Self {
            Self {
                count: Err("no value supplied for count".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl SpectatorsMsg {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SpectatorsMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SpectatorsMsg> for super::SpectatorsMsg {
        type Error = String;
        fn try_from(value: SpectatorsMsg) -> Result<Self, String> {
            Ok(Self {
                count: value.count?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::SpectatorsMsg> for SpectatorsMsg {
        fn from(value: super::SpectatorsMsg) -> Self {
            Self {
                count: Ok(value.count),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateMsg {
        id: Result<i64, String>,
        reset: Result<Option<bool>, String>,
        score: Result<i64, String>,
        type_: Result<super::UpdateMsgType, String>,
        used: Result<Vec<bool>, String>,
    }
    impl Default for UpdateMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                reset: Ok(Default::default()),
                score: Err("no value supplied for score".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                used: Ok(Default::default()),
            }
        }
    }
    impl UpdateMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn reset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.reset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reset: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UpdateMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<bool>>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UpdateMsg> for super::UpdateMsg {
        type Error = String;
        fn try_from(value: UpdateMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                reset: value.reset?,
                score: value.score?,
                type_: value.type_?,
                used: value.used?,
            })
        }
    }
    impl From<super::UpdateMsg> for UpdateMsg {
        fn from(value: super::UpdateMsg) -> Self {
            Self {
                id: Ok(value.id),
                reset: Ok(value.reset),
                score: Ok(value.score),
                type_: Ok(value.type_),
                used: Ok(value.used),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateNameMsg {
        id: Result<i64, String>,
        name: Result<String, String>,
        type_: Result<super::UpdateNameMsgType, String>,
    }
    impl Default for UpdateNameMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UpdateNameMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UpdateNameMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UpdateNameMsg> for super::UpdateNameMsg {
        type Error = String;
        fn try_from(value: UpdateNameMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::UpdateNameMsg> for UpdateNameMsg {
        fn from(value: super::UpdateNameMsg) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateTurnMsg {
        id: Result<i64, String>,
        skip: Result<Option<bool>, String>,
        type_: Result<super::UpdateTurnMsgType, String>,
    }
    impl Default for UpdateTurnMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                skip: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl UpdateTurnMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn skip<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.skip = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skip: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::UpdateTurnMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UpdateTurnMsg> for super::UpdateTurnMsg {
        type Error = String;
        fn try_from(value: UpdateTurnMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                skip: value.skip?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::UpdateTurnMsg> for UpdateTurnMsg {
        fn from(value: super::UpdateTurnMsg) -> Self {
            Self {
                id: Ok(value.id),
                skip: Ok(value.skip),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserData {
        achievements: Result<Option<Vec<super::Achievement>>, String>,
        color: Result<super::Color, String>,
        created_date: Result<chrono::DateTime<chrono::Utc>, String>,
        dice: Result<super::Dice, String>,
        donor: Result<bool, String>,
        id: Result<String, String>,
        image_url: Result<Option<String>, String>,
        pubkey_text: Result<Option<String>, String>,
        stats: Result<Option<super::UserStats>, String>,
        username: Result<String, String>,
    }
    impl Default for UserData {
        fn default() -> Self {
            Self {
                achievements: Ok(Default::default()),
                color: Err("no value supplied for color".to_string()),
                created_date: Err("no value supplied for created_date".to_string()),
                dice: Err("no value supplied for dice".to_string()),
                donor: Err("no value supplied for donor".to_string()),
                id: Err("no value supplied for id".to_string()),
                image_url: Ok(Default::default()),
                pubkey_text: Ok(Default::default()),
                stats: Ok(Default::default()),
                username: Err("no value supplied for username".to_string()),
            }
        }
    }
    impl UserData {
        pub fn achievements<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<super::Achievement>>>,
            T::Error: std::fmt::Display,
        {
            self.achievements = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for achievements: {}", e));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Color>,
            T::Error: std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn created_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.created_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_date: {}", e));
            self
        }
        pub fn dice<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Dice>,
            T::Error: std::fmt::Display,
        {
            self.dice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dice: {}", e));
            self
        }
        pub fn donor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.donor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for donor: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {}", e));
            self
        }
        pub fn pubkey_text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pubkey_text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pubkey_text: {}", e));
            self
        }
        pub fn stats<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::UserStats>>,
            T::Error: std::fmt::Display,
        {
            self.stats = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stats: {}", e));
            self
        }
        pub fn username<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.username = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for username: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserData> for super::UserData {
        type Error = String;
        fn try_from(value: UserData) -> Result<Self, String> {
            Ok(Self {
                achievements: value.achievements?,
                color: value.color?,
                created_date: value.created_date?,
                dice: value.dice?,
                donor: value.donor?,
                id: value.id?,
                image_url: value.image_url?,
                pubkey_text: value.pubkey_text?,
                stats: value.stats?,
                username: value.username?,
            })
        }
    }
    impl From<super::UserData> for UserData {
        fn from(value: super::UserData) -> Self {
            Self {
                achievements: Ok(value.achievements),
                color: Ok(value.color),
                created_date: Ok(value.created_date),
                dice: Ok(value.dice),
                donor: Ok(value.donor),
                id: Ok(value.id),
                image_url: Ok(value.image_url),
                pubkey_text: Ok(value.pubkey_text),
                stats: Ok(value.stats),
                username: Ok(value.username),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UserStats {
        doubles: Result<i64, String>,
        games: Result<i64, String>,
        rolls: Result<i64, String>,
        wins: Result<i64, String>,
    }
    impl Default for UserStats {
        fn default() -> Self {
            Self {
                doubles: Err("no value supplied for doubles".to_string()),
                games: Err("no value supplied for games".to_string()),
                rolls: Err("no value supplied for rolls".to_string()),
                wins: Err("no value supplied for wins".to_string()),
            }
        }
    }
    impl UserStats {
        pub fn doubles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.doubles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for doubles: {}", e));
            self
        }
        pub fn games<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.games = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for games: {}", e));
            self
        }
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn wins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.wins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wins: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<UserStats> for super::UserStats {
        type Error = String;
        fn try_from(value: UserStats) -> Result<Self, String> {
            Ok(Self {
                doubles: value.doubles?,
                games: value.games?,
                rolls: value.rolls?,
                wins: value.wins?,
            })
        }
    }
    impl From<super::UserStats> for UserStats {
        fn from(value: super::UserStats) -> Self {
            Self {
                doubles: Ok(value.doubles),
                games: Ok(value.games),
                rolls: Ok(value.rolls),
                wins: Ok(value.wins),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WelcomeMsg {
        chat_log: Result<Vec<String>, String>,
        id: Result<i64, String>,
        players: Result<Vec<super::Player>, String>,
        private_session: Result<bool, String>,
        rolled: Result<bool, String>,
        rolls: Result<Vec<i64>, String>,
        spectators: Result<i64, String>,
        turn_index: Result<i64, String>,
        type_: Result<super::WelcomeMsgType, String>,
        used: Result<Vec<bool>, String>,
        victory: Result<bool, String>,
    }
    impl Default for WelcomeMsg {
        fn default() -> Self {
            Self {
                chat_log: Err("no value supplied for chat_log".to_string()),
                id: Err("no value supplied for id".to_string()),
                players: Err("no value supplied for players".to_string()),
                private_session: Err("no value supplied for private_session".to_string()),
                rolled: Err("no value supplied for rolled".to_string()),
                rolls: Err("no value supplied for rolls".to_string()),
                spectators: Err("no value supplied for spectators".to_string()),
                turn_index: Err("no value supplied for turn_index".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                used: Err("no value supplied for used".to_string()),
                victory: Err("no value supplied for victory".to_string()),
            }
        }
    }
    impl WelcomeMsg {
        pub fn chat_log<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.chat_log = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chat_log: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn players<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Player>>,
            T::Error: std::fmt::Display,
        {
            self.players = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for players: {}", e));
            self
        }
        pub fn private_session<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.private_session = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for private_session: {}", e));
            self
        }
        pub fn rolled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.rolled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolled: {}", e));
            self
        }
        pub fn rolls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rolls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rolls: {}", e));
            self
        }
        pub fn spectators<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.spectators = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spectators: {}", e));
            self
        }
        pub fn turn_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.turn_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for turn_index: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::WelcomeMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn used<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<bool>>,
            T::Error: std::fmt::Display,
        {
            self.used = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for used: {}", e));
            self
        }
        pub fn victory<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.victory = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for victory: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<WelcomeMsg> for super::WelcomeMsg {
        type Error = String;
        fn try_from(value: WelcomeMsg) -> Result<Self, String> {
            Ok(Self {
                chat_log: value.chat_log?,
                id: value.id?,
                players: value.players?,
                private_session: value.private_session?,
                rolled: value.rolled?,
                rolls: value.rolls?,
                spectators: value.spectators?,
                turn_index: value.turn_index?,
                type_: value.type_?,
                used: value.used?,
                victory: value.victory?,
            })
        }
    }
    impl From<super::WelcomeMsg> for WelcomeMsg {
        fn from(value: super::WelcomeMsg) -> Self {
            Self {
                chat_log: Ok(value.chat_log),
                id: Ok(value.id),
                players: Ok(value.players),
                private_session: Ok(value.private_session),
                rolled: Ok(value.rolled),
                rolls: Ok(value.rolls),
                spectators: Ok(value.spectators),
                turn_index: Ok(value.turn_index),
                type_: Ok(value.type_),
                used: Ok(value.used),
                victory: Ok(value.victory),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WinMsg {
        id: Result<i64, String>,
        type_: Result<super::WinMsgType, String>,
    }
    impl Default for WinMsg {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl WinMsg {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::WinMsgType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<WinMsg> for super::WinMsg {
        type Error = String;
        fn try_from(value: WinMsg) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::WinMsg> for WinMsg {
        fn from(value: super::WinMsg) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
            }
        }
    }
}
pub mod defaults {
    pub(super) fn room_list_msg_type() -> super::RoomListMsgType {
        super::RoomListMsgType::RoomList
    }
}
