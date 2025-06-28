pub struct KNodeMsg {
    sender: u8,
    receiver: u8,
    kind: KNodeMsgKind,
    payload: Option<KNodePayload>,
}

pub enum KNodeMsgKind {
    Command(KNodeCommand),
    Response(KNodeResponse),
    Heartbeat,
    Err(KNodeErr),
}

pub enum KNodePayload {
    ControlData(f32),
    Initilize(f32),
    Info { id: usize, data: [u8, 32], len: usize },
}

pub enum KNodeErr {
    WrongState,
    Fault,
    Ok,
}

pub enum KNodeCommand {
    Initialize,
    SetPosition,
    SetVelocity,
    SetTorque,
    GetData,
}

pub enum KNodeResponse {
    Initilized,
    PositionSet,
    VelocitySet,
    TorqueSet,
    DataSent,
}
