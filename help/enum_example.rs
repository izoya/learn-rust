enum SpaceMissionStage {
    LaunchPreparation,
    Liftoff,
    InOrbit,
    LunarLanding,
    MissionAccomplished,
}


enum Result<T, E> {
    Ok(T),
    Err(E),
}


enum Option<T> {
    Some(T),
    None,
}

