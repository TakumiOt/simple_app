use crate::entities::SensorData;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SensorRepository: Send + Sync {
    async fn save(&self, data: &SensorData) -> Result<()>;

    async fn find_by_device_id(&self, device_id: &str) -> Result<Vec<SensorData>>;
}
