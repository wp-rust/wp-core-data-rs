use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use wp_api_fetch_rs::ApiFetch;

pub struct CoreDataStore<'a> {
    api_fetch: &'a ApiFetch,
    cache: RwLock<HashMap<String, Value>>,
}

impl<'a> CoreDataStore<'a> {
    pub fn new(api_fetch: &'a ApiFetch) -> Self {
        Self {
            api_fetch,
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_entity_records(&self, kind: &str, name: &str) -> Result<Value> {
        let cache_key = format!("{}/{}/records", kind, name);
        
        {
            let cache = self.cache.read().unwrap();
            if let Some(data) = cache.get(&cache_key) {
                return Ok(data.clone());
            }
        }

        let path = format!("/{}/{}", kind, name);
        let res = self.api_fetch.get(&path).await?;
        
        // Ensure success status before parsing
        let res = res.error_for_status()?;
        let data: Value = res.json().await?;

        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(cache_key, data.clone());
        }

        Ok(data)
    }

    pub async fn get_entity_record(&self, kind: &str, name: &str, id: i64) -> Result<Value> {
        let cache_key = format!("{}/{}/records/{}", kind, name, id);
        
        {
            let cache = self.cache.read().unwrap();
            if let Some(data) = cache.get(&cache_key) {
                return Ok(data.clone());
            }
        }

        let path = format!("/{}/{}/{}", kind, name, id);
        let res = self.api_fetch.get(&path).await?;
        
        let res = res.error_for_status()?;
        let data: Value = res.json().await?;

        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(cache_key, data.clone());
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wp_api_fetch_rs::Auth;

    #[test]
    fn test_core_data_store_creation() {
        let api = ApiFetch::new("https://example.com/wp-json", Auth::None);
        let store = CoreDataStore::new(&api);
        
        let cache = store.cache.read().unwrap();
        assert!(cache.is_empty());
    }
}
