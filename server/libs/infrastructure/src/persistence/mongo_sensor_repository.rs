use crate::persistence::models::SensorDataDocument;
use anyhow::Result;
use async_trait::async_trait;
use domain::entities::SensorData;
use domain::repositories::SensorRepository;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;

pub struct MongoSensorRepository {
    collection: Collection<SensorDataDocument>,
}

impl MongoSensorRepository {
    pub fn new(collection: Collection<SensorDataDocument>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl SensorRepository for MongoSensorRepository {
    async fn save(&self, data: &SensorData) -> Result<()> {
        let document = SensorDataDocument::from(data);
        self.collection.insert_one(document).await?;
        Ok(())
    }

    async fn find_by_device_id(&self, device_id: &str) -> Result<Vec<SensorData>> {
        let filter = doc! { "device_id": device_id };
        let cursor = self.collection.find(filter).await?;
        let documents: Vec<SensorDataDocument> = cursor.try_collect().await?;
        let sensor_data = documents.into_iter().map(SensorData::from).collect();
        Ok(sensor_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use mongodb::Client;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn load_env() {
        INIT.call_once(|| {
            dotenvy::dotenv().ok();
        });
    }

    async fn setup_test_repository(collection_name: &str) -> (MongoSensorRepository, Collection<SensorDataDocument>) {
        load_env();
        let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client = Client::with_uri_str(&uri).await.expect("Failed to connect to MongoDB");
        let db = client.database("sensor_test_db");
        let collection = db.collection::<SensorDataDocument>(collection_name);

        // テスト前にコレクションをクリア
        collection.drop().await.ok();

        (MongoSensorRepository::new(collection.clone()), collection)
    }

    #[tokio::test]
    async fn test_save_sensor_data() {
        let (repo, collection) = setup_test_repository("test_save").await;

        let data = SensorData::new("device-001".to_string(), Utc::now())
            .with_temperature(25.5, "celsius");

        let result = repo.save(&data).await;
        assert!(result.is_ok());

        // データが保存されたことを確認
        let count = collection.count_documents(doc! {}).await.unwrap();
        assert_eq!(count, 1);

        // クリーンアップ
        collection.drop().await.ok();
    }

    #[tokio::test]
    async fn test_find_by_device_id() {
        let (repo, collection) = setup_test_repository("test_find").await;

        let device_id = "device-002";
        let data1 = SensorData::new(device_id.to_string(), Utc::now())
            .with_temperature(20.0, "celsius");
        let data2 = SensorData::new(device_id.to_string(), Utc::now())
            .with_humidity(55.0, "percent");
        let data3 = SensorData::new("other-device".to_string(), Utc::now())
            .with_co2(400.0, "ppm");

        repo.save(&data1).await.unwrap();
        repo.save(&data2).await.unwrap();
        repo.save(&data3).await.unwrap();

        let results = repo.find_by_device_id(device_id).await.unwrap();

        assert_eq!(results.len(), 2);
        for result in &results {
            assert_eq!(result.device_id, device_id);
        }

        // クリーンアップ
        collection.drop().await.ok();
    }

    #[tokio::test]
    async fn test_find_by_device_id_not_found() {
        let (repo, collection) = setup_test_repository("test_not_found").await;

        let results = repo.find_by_device_id("non-existent-device").await.unwrap();

        assert!(results.is_empty());

        // クリーンアップ
        collection.drop().await.ok();
    }

    #[tokio::test]
    async fn test_save_with_all_sensor_types() {
        let (repo, collection) = setup_test_repository("test_all_sensors").await;

        let data = SensorData::new("device-003".to_string(), Utc::now())
            .with_temperature(22.5, "celsius")
            .with_humidity(60.0, "percent")
            .with_co2(450.0, "ppm")
            .with_additional_sensor("pressure", 1013.25, "hPa");

        repo.save(&data).await.unwrap();

        let results = repo.find_by_device_id("device-003").await.unwrap();
        assert_eq!(results.len(), 1);

        let saved = &results[0];
        assert!(saved.temperature.is_some());
        assert!(saved.humidity.is_some());
        assert!(saved.co2.is_some());
        assert!(saved.additional_sensors.contains_key("pressure"));

        let temp = saved.temperature.as_ref().unwrap();
        assert_eq!(temp.value, 22.5);
        assert_eq!(temp.unit, "celsius");

        // クリーンアップ
        collection.drop().await.ok();
    }
}
