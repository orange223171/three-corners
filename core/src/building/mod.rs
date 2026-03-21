//! Содержит определения для работы с конкретным экземпляром строения

/// Строение
pub struct Building {
    /// Название строения
    pub name: String,
    /// Игрок, установивший строение
    pub player: String,

    /// Строение построено в текущий раунд
    pub build_in_current_round: bool,

    /// Синергии, в которых состоит строение
    pub synergies: Vec<String>,
}
