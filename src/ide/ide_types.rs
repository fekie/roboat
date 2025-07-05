use bytes::Bytes;
#[allow(missing_docs)]
#[derive(Clone, Debug)]
/// An struct to Make a new animation
pub struct NewAnimation {
    pub name: String,
    pub description: String,
    pub group_id: Option<u64>,
    pub animation_data: Bytes,
}
