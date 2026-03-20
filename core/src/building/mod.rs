/// Строение
pub struct Building {
    /// Название строения
    name: String,
    /// Игрок, установивший строение
    player: String,

    /// Строение построено в текущий раунд
    build_in_current_round: bool,

    //Синергии, в которых состоит строение
    synergies: Vec<String>,
}
